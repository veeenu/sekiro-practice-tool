use std::fmt::Write;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use const_format::formatcp;
use hudhook::imgui::*;
use hudhook::tracing::metadata::LevelFilter;
use hudhook::tracing::{error, info};
use hudhook::{ImguiRenderLoop, RenderContext};
use libsekiro::pointers::Pointers;
use libsekiro::version;
use pkg_version::*;
use practice_tool_core::crossbeam_channel::{self, Receiver, Sender};
use practice_tool_core::widgets::{scaling_factor, Widget, BUTTON_HEIGHT, BUTTON_WIDTH};
use tracing_subscriber::prelude::*;

use crate::config::{Config, IndicatorType, Settings};
use crate::util;

const MAJOR: usize = pkg_version_major!();
const MINOR: usize = pkg_version_minor!();
const PATCH: usize = pkg_version_patch!();

struct FontIDs {
    small: FontId,
    normal: FontId,
    big: FontId,
}

unsafe impl Send for FontIDs {}
unsafe impl Sync for FontIDs {}

enum UiState {
    MenuOpen,
    Closed,
    Hidden,
}

pub struct PracticeTool {
    pointers: Pointers,
    settings: Settings,
    version_label: String,
    widgets: Vec<Box<dyn Widget>>,

    log: Vec<(Instant, String)>,
    log_rx: Receiver<String>,
    log_tx: Sender<String>,
    ui_state: UiState,
    fonts: Option<FontIDs>,

    position_bufs: [String; 3],
    position_prev: [f32; 3],
    position_change_buf: String,

    igt_buf: String,

    fps_buf: String,

    framecount: u32,
    framecount_buf: String,

    config_err: Option<String>,
}

impl PracticeTool {
    pub fn new() -> Self {
        hudhook::alloc_console().ok();
        hudhook::enable_console_colors();

        fn load_config() -> Result<Config, String> {
            let config_path = util::get_dll_path()
                .map(|mut path| {
                    path.pop();
                    path.push("jdsd_sekiro_practice_tool.toml");
                    path
                })
                .ok_or_else(|| "Couldn't find config file".to_string())?;
            let config_content = std::fs::read_to_string(config_path)
                .map_err(|e| format!("Couldn't read config file: {}", e))?;
            println!("{}", config_content);
            Config::parse(&config_content).map_err(String::from)
        }

        let (config, config_err) = match load_config() {
            Ok(config) => (config, None),
            Err(e) => (
                Config::default(),
                Some({
                    error!("{}", e);
                    format!(
                        "Configuration error, please review your jdsd_er_practice_tool.toml \
                         file.\n\n{e}"
                    )
                }),
            ),
        };

        let log_file = util::get_dll_path()
            .map(|mut path| {
                path.pop();
                path.push("jdsd_sekiro_practice_tool.log");
                path
            })
            .map(std::fs::File::create);

        let log_level = config.settings.log_level.inner();

        if log_level < LevelFilter::DEBUG || !config.settings.show_console {
            hudhook::free_console().ok();
        }

        match log_file {
            Some(Ok(log_file)) => {
                let file_layer = tracing_subscriber::fmt::layer()
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true)
                    .with_writer(Mutex::new(log_file))
                    .with_ansi(false)
                    .boxed();
                let stdout_layer = tracing_subscriber::fmt::layer()
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_thread_names(true)
                    .with_ansi(true)
                    .boxed();

                tracing_subscriber::registry()
                    .with(log_level)
                    .with(file_layer)
                    .with(stdout_layer)
                    .init();
            },
            e => match e {
                None => error!("Could not construct log file path"),
                Some(Err(e)) => error!("Could not initialize log file: {:?}", e),
                _ => unreachable!(),
            },
        }

        let pointers = Pointers::new();
        let settings = config.settings.clone();
        let widgets = config.make_commands(&pointers);

        let version_label = {
            let (maj, min, patch) = version::get_version().into();
            format!("Game Ver {}.{:02}.{}", maj, min, patch)
        };
        let (log_tx, log_rx) = crossbeam_channel::unbounded();
        info!("Practice tool initialized");

        PracticeTool {
            pointers,
            settings,
            version_label,
            widgets,
            ui_state: UiState::Closed,
            log: Default::default(),
            fonts: None,
            config_err,
            log_rx,
            log_tx,
            position_prev: Default::default(),
            position_bufs: Default::default(),
            position_change_buf: Default::default(),
            igt_buf: Default::default(),
            fps_buf: Default::default(),
            framecount: 0,
            framecount_buf: Default::default(),
        }
    }

    fn render_visible(&mut self, ui: &Ui) {
        let [dw, dh] = { ui.io().display_size };
        ui.window("##tool_window")
            .position([16., 16.], Condition::Always)
            .size_constraints([240., 0.], [dw - 70., dh - 70.])
            .bg_alpha(0.8)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(|| {
                if let Some(e) = self.config_err.as_ref() {
                    ui.text(e);
                }

                for w in self.widgets.iter_mut() {
                    w.render(ui);
                }
                if !ui.io().want_capture_keyboard {
                    for w in self.widgets.iter_mut() {
                        w.interact(ui);
                    }
                }

                if ui.button_with_size("Close", [BUTTON_WIDTH * scaling_factor(ui), BUTTON_HEIGHT])
                {
                    self.ui_state = UiState::Closed;
                    // self.pointers.show_cursor.set(false);
                }

                if option_env!("CARGO_XTASK_DIST").is_none()
                    && ui.button_with_size("Eject", [
                        BUTTON_WIDTH * scaling_factor(ui),
                        BUTTON_HEIGHT,
                    ])
                {
                    self.ui_state = UiState::Closed;
                    self.pointers.show_cursor.set(false);
                    hudhook::eject();
                }
            });
    }

    fn render_closed(&mut self, ui: &Ui) {
        let [w, h] = ui.io().display_size;

        let stack_tokens = vec![
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];
        ui.window("##msg_window")
            .position([w * 35. / 1920., h * 112. / 1080.], Condition::Always)
            .bg_alpha(0.0)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(|| {
                ui.text("johndisandonato's Practice Tool");

                if ui.small_button("Open") {
                    self.ui_state = UiState::MenuOpen;
                }

                ui.same_line();

                if ui.small_button("Indicators") {
                    ui.open_popup("##indicators_window");
                }

                ui.modal_popup_config("##indicators_window")
                    .resizable(false)
                    .movable(false)
                    .title_bar(false)
                    .build(|| {
                        let style = ui.clone_style();

                        self.pointers.show_cursor.set(true);

                        ui.text(
                            "You can toggle indicators here, as\nwell as reset the frame \
                             counter.\n\nKeep in mind that the available\nindicators and order of \
                             them depend\non your config file.",
                        );
                        ui.separator();

                        for indicator in &mut self.settings.indicators {
                            let label = match indicator.indicator {
                                IndicatorType::GameVersion => "Game Version",
                                IndicatorType::Position => "Player Position",
                                IndicatorType::PositionChange => "Player Velocity",
                                IndicatorType::Igt => "IGT Timer",
                                IndicatorType::Fps => "FPS",
                                IndicatorType::FrameCount => "Frame Counter",
                                IndicatorType::ImguiDebug => "ImGui Debug Info",
                            };

                            let mut state = indicator.enabled;

                            if ui.checkbox(label, &mut state) {
                                indicator.enabled = state;
                            }

                            if let IndicatorType::FrameCount = indicator.indicator {
                                ui.same_line();

                                let btn_reset_label = "Reset";
                                let btn_reset_width = ui.calc_text_size(btn_reset_label)[0]
                                    + style.frame_padding[0] * 2.0;

                                ui.set_cursor_pos([
                                    ui.content_region_max()[0] - btn_reset_width,
                                    ui.cursor_pos()[1],
                                ]);

                                if ui.button("Reset") {
                                    self.framecount = 0;
                                }
                            }
                        }

                        ui.separator();

                        let btn_close_width =
                            ui.content_region_max()[0] - style.frame_padding[0] * 2.0;

                        if ui.button_with_size("Close", [btn_close_width, 0.0]) {
                            ui.close_current_popup();
                            self.pointers.show_cursor.set(false);
                        }
                    });

                ui.same_line();

                if ui.small_button("Help") {
                    ui.open_popup("##help_window");
                }

                ui.modal_popup_config("##help_window")
                    .resizable(false)
                    .movable(false)
                    .title_bar(false)
                    .build(|| {
                        // self.pointers.show_cursor.set(true);
                        ui.text(formatcp!("Sekiro Practice Tool v{}.{}.{}", MAJOR, MINOR, PATCH));
                        ui.separator();
                        ui.text(format!(
                            "Press the {} key to open/close the tool's\ninterface.\n\nYou can \
                             toggle flags/launch commands by\nclicking in the UI or by \
                             pressing\nthe hotkeys (in the parentheses).\n\nYou can configure \
                             your tool by editing\nthe jdsd_er_practice_tool.toml file with\na \
                             text editor. If you break something,\njust download a fresh \
                             file!\n\nThank you for using my tool! <3\n",
                            self.settings.display
                        ));
                        ui.separator();
                        ui.text("-- johndisandonato");
                        ui.text("   https://twitch.tv/johndisandonato");
                        if ui.is_item_clicked() {
                            open::that("https://twitch.tv/johndisandonato").ok();
                        }
                        ui.separator();
                        if ui.button("Close") {
                            ui.close_current_popup();
                            // self.pointers.show_cursor.set(false);
                        }
                        ui.same_line();
                        if ui.button("Submit issue") {
                            open::that("https://github.com/veeenu/sekiro-practice-tool/issues/new")
                                .ok();
                        }
                        ui.same_line();
                        if ui.button("Support") {
                            open::that("https://patreon.com/johndisandonato").ok();
                        }
                    });

                ui.new_line();

                for indicator in &self.settings.indicators {
                    if !indicator.enabled {
                        continue;
                    }

                    match indicator.indicator {
                        IndicatorType::GameVersion => {
                            ui.text(&self.version_label);
                        },
                        IndicatorType::Position => {
                            if let Some([x, y, z, _]) = self.pointers.position.read() {
                                self.position_bufs.iter_mut().for_each(String::clear);
                                write!(self.position_bufs[0], "{x:.3}").ok();
                                write!(self.position_bufs[1], "{y:.3}").ok();
                                write!(self.position_bufs[2], "{z:.3}").ok();

                                ui.text_colored(
                                    [0.7048, 0.1228, 0.1734, 1.],
                                    &self.position_bufs[0],
                                );
                                ui.same_line();
                                ui.text_colored(
                                    [0.1161, 0.5327, 0.3512, 1.],
                                    &self.position_bufs[1],
                                );
                                ui.same_line();
                                ui.text_colored(
                                    [0.1445, 0.2852, 0.5703, 1.],
                                    &self.position_bufs[2],
                                );
                            }
                        },
                        IndicatorType::PositionChange => {
                            if let Some([x, y, z, _]) = self.pointers.position.read() {
                                let position_change_xyz = ((x - self.position_prev[0]).powf(2.0)
                                    + (y - self.position_prev[1]).powf(2.0)
                                    + (z - self.position_prev[2]).powf(2.0))
                                .sqrt();

                                let position_change_xz = ((x - self.position_prev[0]).powf(2.0)
                                    + (z - self.position_prev[2]).powf(2.0))
                                .sqrt();

                                let position_change_y = y - self.position_prev[1];

                                self.position_change_buf.clear();
                                write!(
                                    self.position_change_buf,
                                    "[XYZ] {position_change_xyz:.6} | [XZ] \
                                     {position_change_xz:.6} | [Y] {position_change_y:.6}"
                                )
                                .ok();
                                ui.text(&self.position_change_buf);

                                self.position_prev = [x, y, z];
                            }
                        },
                        IndicatorType::Igt => {
                            if let Some(igt) = self.pointers.igt.read() {
                                let millis = (igt % 1000) / 10;
                                let total_seconds = igt / 1000;
                                let seconds = total_seconds % 60;
                                let minutes = total_seconds / 60 % 60;
                                let hours = total_seconds / 3600;
                                self.igt_buf.clear();
                                write!(
                                    self.igt_buf,
                                    "IGT {hours:02}:{minutes:02}:{seconds:02}.{millis:02}",
                                )
                                .ok();
                                ui.text(&self.igt_buf);
                            }
                        },
                        IndicatorType::Fps => {
                            if let Some(fps) = self.pointers.fps.read() {
                                self.fps_buf.clear();
                                write!(self.fps_buf, "FPS {fps}",).ok();
                                ui.text(&self.fps_buf);
                            }
                        },
                        IndicatorType::FrameCount => {
                            self.framecount_buf.clear();
                            write!(self.framecount_buf, "Frame count {0}", self.framecount,).ok();
                            ui.text(&self.framecount_buf);
                        },
                        IndicatorType::ImguiDebug => {
                            imgui_debug(ui);
                        },
                    }
                }

                if !ui.io().want_capture_keyboard {
                    for w in self.widgets.iter_mut() {
                        w.interact(ui);
                    }
                }
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }

    fn render_hidden(&mut self, ui: &Ui) {
        if !ui.io().want_capture_keyboard {
            for w in self.widgets.iter_mut() {
                w.interact(ui);
            }
        }
    }

    fn render_logs(&mut self, ui: &Ui) {
        let io = ui.io();

        let [dw, dh] = io.display_size;
        let [ww, wh] = [dw * 0.3, 14.0 * 6.];

        let stack_tokens = vec![
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];

        ui.window("##logs")
            .position_pivot([1., 1.])
            .position([dw * 0.95, dh * 0.8], Condition::Always)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .size([ww, wh], Condition::Always)
            .bg_alpha(0.0)
            .build(|| {
                for _ in 0..20 {
                    ui.text("");
                }
                for l in self.log.iter() {
                    ui.text(&l.1);
                }
                ui.set_scroll_here_y();
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }

    fn set_font<'a>(&mut self, ui: &'a Ui) -> FontStackToken<'a> {
        let width = ui.io().display_size[0];
        let font_id = self
            .fonts
            .as_mut()
            .map(|fonts| {
                if width > 2000. {
                    fonts.big
                } else if width > 1200. {
                    fonts.normal
                } else {
                    fonts.small
                }
            })
            .unwrap();

        ui.push_font(font_id)
    }
}

impl ImguiRenderLoop for PracticeTool {
    fn render(&mut self, ui: &mut Ui) {
        let font_token = self.set_font(ui);

        let display = self.settings.display.is_pressed(ui);
        let hide = self.settings.hide.map(|k| k.is_pressed(ui)).unwrap_or(false);

        self.framecount += 1;

        if !ui.io().want_capture_keyboard && (display || hide) {
            self.ui_state = match (&self.ui_state, hide) {
                (UiState::Hidden, _) => UiState::Closed,
                (_, true) => UiState::Hidden,
                (UiState::MenuOpen, _) => UiState::Closed,
                (UiState::Closed, _) => UiState::MenuOpen,
            };

            // match &self.ui_state {
            //     UiState::MenuOpen => {},
            //     UiState::Closed => self.pointers.show_cursor.set(false),
            //     UiState::Hidden => self.pointers.show_cursor.set(false),
            // }
        }

        match &self.ui_state {
            UiState::MenuOpen => {
                // self.pointers.show_cursor.set(true);
                self.render_visible(ui);
            },
            UiState::Closed => {
                self.render_closed(ui);
            },
            UiState::Hidden => {
                self.render_hidden(ui);
            },
        }

        for w in &mut self.widgets {
            w.log(self.log_tx.clone());
        }

        let now = Instant::now();
        self.log.extend(self.log_rx.try_iter().inspect(|log| info!("{}", log)).map(|l| (now, l)));
        self.log.retain(|(tm, _)| tm.elapsed() < Duration::from_secs(5));

        self.render_logs(ui);
        drop(font_token);
    }

    fn initialize(&mut self, ctx: &mut Context, _: &mut dyn RenderContext) {
        let fonts = ctx.fonts();
        self.fonts = Some(FontIDs {
            small: fonts.add_font(&[FontSource::TtfData {
                data: include_bytes!("../data/ComicMono.ttf"),
                size_pixels: 11.,
                config: None,
            }]),
            normal: fonts.add_font(&[FontSource::TtfData {
                data: include_bytes!("../data/ComicMono.ttf"),
                size_pixels: 18.,
                config: None,
            }]),
            big: fonts.add_font(&[FontSource::TtfData {
                data: include_bytes!("../data/ComicMono.ttf"),
                size_pixels: 24.,
                config: None,
            }]),
        });
    }
}

// Display some imgui debug information. Very expensive.
fn imgui_debug(ui: &Ui) {
    let io = ui.io();
    ui.text(format!("Mouse position     {:?}", io.mouse_pos));
    ui.text(format!("Mouse down         {:?}", io.mouse_down));
    ui.text(format!("Want capture mouse {:?}", io.want_capture_mouse));
    ui.text(format!("Want capture kbd   {:?}", io.want_capture_keyboard));
    ui.text(format!("Want text input    {:?}", io.want_text_input));
    ui.text(format!("Want set mouse pos {:?}", io.want_set_mouse_pos));
    ui.text(format!("Any item active    {:?}", ui.is_any_item_active()));
    ui.text(format!("Any item hovered   {:?}", ui.is_any_item_hovered()));
    ui.text(format!("Any item focused   {:?}", ui.is_any_item_focused()));
    ui.text(format!("Any mouse down     {:?}", ui.is_any_mouse_down()));
}
