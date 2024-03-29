// johndisandonato's Sekiro Practice Tool
// Copyright (C) 2023  johndisandonato <https://github.com/veeenu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
#![feature(lazy_cell)]

mod config;
mod widgets;

use std::sync::Mutex;
use std::time::Instant;

use config::Config;
use const_format::formatcp;
use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::hooks::{ImguiRenderLoop, ImguiRenderLoopFlags};
use hudhook::imgui::{self, *};
use hudhook::tracing::metadata::LevelFilter;
use hudhook::tracing::{error, info};
use libsekiro::prelude::Pointers;
use libsekiro::version::VERSION;
use pkg_version::*;
use practice_tool_utils::widgets::{scaling_factor, Widget, BUTTON_HEIGHT, BUTTON_WIDTH};
use tracing_subscriber::prelude::*;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_RSHIFT};

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

struct PracticeTool {
    pointers: Pointers,
    config: Config,
    version_label: String,
    widgets: Vec<Box<dyn Widget>>,
    log: Vec<(Instant, String)>,
    ui_state: UiState,
    fonts: Option<FontIDs>,
    config_err: Option<String>,
}

impl PracticeTool {
    fn new() -> Self {
        hudhook::utils::alloc_console();
        hudhook::utils::enable_console_colors();

        fn load_config() -> Result<Config, String> {
            let config_path = practice_tool_utils::get_dll_path()
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

        let log_file = practice_tool_utils::get_dll_path()
            .map(|mut path| {
                path.pop();
                path.push("jdsd_sekiro_practice_tool.log");
                path
            })
            .map(std::fs::File::create);

        let log_level = config.settings.log_level.inner();

        if log_level < LevelFilter::DEBUG || !config.settings.show_console {
            hudhook::utils::free_console();
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

        if config.settings.dxgi_debug {
            hudhook::hooks::dx12::enable_dxgi_debug();
        }

        let pointers = Pointers::new();
        let widgets = config.make_commands(&pointers);

        let version_label = {
            let (maj, min, patch) = VERSION.tuple();
            format!("Ver {}.{:02}.{}", maj, min, patch)
        };
        info!("Practice tool initialized");

        PracticeTool {
            pointers,
            config,
            version_label,
            widgets,
            ui_state: UiState::Closed,
            log: Default::default(),
            fonts: None,
            config_err,
        }
    }

    fn render_visible(&mut self, ui: &imgui::Ui, flags: &ImguiRenderLoopFlags) {
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
                if flags.focused && !ui.io().want_capture_keyboard {
                    for w in self.widgets.iter_mut() {
                        w.interact(ui);
                    }
                }

                if ui.button_with_size("Close", [BUTTON_WIDTH * scaling_factor(ui), BUTTON_HEIGHT])
                {
                    self.ui_state = UiState::Closed;
                    // self.pointers.show_cursor.set(false);
                    if option_env!("CARGO_XTASK_DIST").is_none() {
                        hudhook::lifecycle::eject();
                    }
                }
            });
    }

    fn render_closed(&mut self, ui: &imgui::Ui, flags: &ImguiRenderLoopFlags) {
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

                ui.same_line();

                if ui.small_button("Open") {
                    self.ui_state = UiState::MenuOpen;
                }

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
                            self.config.settings.display
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
                    });

                ui.text(&self.version_label);

                if let Some([x, y, z, _]) = self.pointers.position.read() {
                    ui.text_colored([0.7048, 0.1228, 0.1734, 1.], format!("{x:.2}"));
                    ui.same_line();
                    ui.text_colored([0.1161, 0.5327, 0.3512, 1.], format!("{y:.2}"));
                    ui.same_line();
                    ui.text_colored([0.1445, 0.2852, 0.5703, 1.], format!("{z:.2}"));
                }

                if let Some(igt) = self.pointers.igt.read() {
                    let millis = (igt % 1000) / 10;
                    let total_seconds = igt / 1000;
                    let seconds = total_seconds % 60;
                    let minutes = total_seconds / 60 % 60;
                    let hours = total_seconds / 3600;
                    ui.text(format!(
                        "IGT {:02}:{:02}:{:02}.{:02}",
                        hours, minutes, seconds, millis
                    ));
                }

                if flags.focused && !ui.io().want_capture_keyboard {
                    for w in self.widgets.iter_mut() {
                        w.interact(ui);
                    }
                }
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }

    fn render_hidden(&mut self, ui: &imgui::Ui, flags: &ImguiRenderLoopFlags) {
        if flags.focused && !ui.io().want_capture_keyboard {
            for w in self.widgets.iter_mut() {
                w.interact(ui);
            }
        }
    }

    fn render_logs(&mut self, ui: &imgui::Ui, _flags: &ImguiRenderLoopFlags) {
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

    fn set_font<'a>(&mut self, ui: &'a imgui::Ui) -> imgui::FontStackToken<'a> {
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
    fn render(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
        let font_token = self.set_font(ui);

        if flags.focused && !ui.io().want_capture_keyboard && self.config.settings.display.keyup(ui)
        {
            let rshift = unsafe { GetAsyncKeyState(VK_RSHIFT.0 as _) < 0 };

            self.ui_state = match (&self.ui_state, rshift) {
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
                self.render_visible(ui, flags);
            },
            UiState::Closed => {
                self.render_closed(ui, flags);
            },
            UiState::Hidden => {
                self.render_hidden(ui, flags);
            },
        }

        for w in &mut self.widgets {
            if let Some(logs) = w.log() {
                let now = Instant::now();
                self.log.extend(logs.into_iter().map(|l| (now, l)));
            }
            self.log.retain(|(tm, _)| tm.elapsed() < std::time::Duration::from_secs(5));
        }

        self.render_logs(ui, flags);
        drop(font_token);
    }

    fn initialize(&mut self, ctx: &mut imgui::Context) {
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

hudhook::hudhook!(PracticeTool::new().into_hook::<ImguiDx11Hooks>());
