use std::str::FromStr;

use hudhook::tracing::metadata::LevelFilter;
use libsekiro::prelude::*;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::Widget;
use serde::Deserialize;

use crate::widgets::cycle_color::cycle_color;
use crate::widgets::cycle_speed::cycle_speed;
use crate::widgets::flag::flag_widget;
use crate::widgets::group::group;
use crate::widgets::label::label_widget;
use crate::widgets::nudge_pos::nudge_position;
use crate::widgets::position::save_position;
use crate::widgets::quitout::quitout;
use crate::widgets::savefile_manager::savefile_manager;

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) settings: Settings,
    commands: Vec<CfgCommand>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Settings {
    pub(crate) log_level: LevelFilterSerde,
    pub(crate) display: Key,
    pub(crate) hide: Option<Key>,
    #[serde(default)]
    pub(crate) show_console: bool,
    #[serde(default = "Indicator::default_set")]
    pub(crate) indicators: Vec<Indicator>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) enum IndicatorType {
    Igt,
    Position,
    PositionChange,
    GameVersion,
    ImguiDebug,
    Fps,
    FrameCount,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "IndicatorConfig")]
pub(crate) struct Indicator {
    pub(crate) indicator: IndicatorType,
    pub(crate) enabled: bool,
}

impl Indicator {
    fn default_set() -> Vec<Indicator> {
        vec![
            Indicator { indicator: IndicatorType::GameVersion, enabled: true },
            Indicator { indicator: IndicatorType::Igt, enabled: true },
            Indicator { indicator: IndicatorType::Position, enabled: false },
            Indicator { indicator: IndicatorType::PositionChange, enabled: false },
            Indicator { indicator: IndicatorType::Fps, enabled: false },
            Indicator { indicator: IndicatorType::FrameCount, enabled: false },
            Indicator { indicator: IndicatorType::ImguiDebug, enabled: false },
        ]
    }
}

#[derive(Debug, Deserialize, Clone)]
struct IndicatorConfig {
    indicator: String,
    enabled: bool,
}

impl TryFrom<IndicatorConfig> for Indicator {
    type Error = String;

    fn try_from(indicator: IndicatorConfig) -> Result<Self, Self::Error> {
        match indicator.indicator.as_str() {
            "igt" => Ok(Indicator { indicator: IndicatorType::Igt, enabled: indicator.enabled }),
            "position" => {
                Ok(Indicator { indicator: IndicatorType::Position, enabled: indicator.enabled })
            },
            "position_change" => Ok(Indicator {
                indicator: IndicatorType::PositionChange,
                enabled: indicator.enabled,
            }),
            "game_version" => {
                Ok(Indicator { indicator: IndicatorType::GameVersion, enabled: indicator.enabled })
            },
            "fps" => Ok(Indicator { indicator: IndicatorType::Fps, enabled: indicator.enabled }),
            "framecount" => {
                Ok(Indicator { indicator: IndicatorType::FrameCount, enabled: indicator.enabled })
            },
            "imgui_debug" => {
                Ok(Indicator { indicator: IndicatorType::ImguiDebug, enabled: indicator.enabled })
            },
            value => Err(format!("Unrecognized indicator: {value}")),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum PlaceholderOption<T> {
    Data(T),
    #[allow(dead_code)]
    Placeholder(bool),
}

impl<T> PlaceholderOption<T> {
    fn into_option(self) -> Option<T> {
        match self {
            PlaceholderOption::Data(d) => Some(d),
            PlaceholderOption::Placeholder(_) => None,
        }
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
#[serde(untagged)]
enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager")]
        hotkey_load: PlaceholderOption<Key>,
    },
    Flag {
        flag: FlagSpec,
        hotkey: Option<Key>,
    },
    Position {
        position: PlaceholderOption<Key>,
        save: Option<Key>,
    },
    CycleSpeed {
        #[serde(rename = "cycle_speed")]
        values: Vec<f32>,
        hotkey: Option<Key>,
    },
    CycleColor {
        #[serde(rename = "cycle_color")]
        cycle_color: Vec<i32>,
        hotkey: Option<Key>,
    },
    Label {
        #[serde(rename = "label")]
        label: String,
    },
    NudgePosition {
        nudge: f32,
        nudge_up: Option<Key>,
        nudge_down: Option<Key>,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: PlaceholderOption<Key>,
    },
    Group {
        #[serde(rename = "group")]
        label: String,
        commands: Vec<CfgCommand>,
    },
}

impl CfgCommand {
    fn into_widget(self, settings: &Settings, chains: &Pointers) -> Box<dyn Widget> {
        match self {
            CfgCommand::Flag { flag, hotkey: key } => {
                flag_widget(&flag.label, (flag.getter)(chains).clone(), key)
            },
            CfgCommand::SavefileManager { hotkey_load: key_load } => {
                savefile_manager(key_load.into_option(), settings.display)
            },
            CfgCommand::Position { position, save } => {
                save_position(chains.position.clone(), position.into_option(), save)
            },
            CfgCommand::Label { label } => label_widget(label.as_str()),
            CfgCommand::NudgePosition { nudge, nudge_up, nudge_down } => {
                nudge_position(chains.position.clone(), nudge, nudge_up, nudge_down)
            },
            CfgCommand::CycleSpeed { values, hotkey } => {
                cycle_speed(values.as_slice(), chains.anim_speed.clone(), hotkey)
            },
            CfgCommand::CycleColor { cycle_color: values, hotkey } => {
                cycle_color(values.as_slice(), chains.debug_color.clone(), hotkey)
            },
            CfgCommand::Quitout { hotkey } => quitout(chains.quitout.clone(), hotkey.into_option()),
            CfgCommand::Group { label, commands } => group(
                label.as_str(),
                commands.into_iter().map(|c| c.into_widget(settings, chains)).collect(),
                settings.display,
            ),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
pub(crate) struct LevelFilterSerde(LevelFilter);

impl LevelFilterSerde {
    pub(crate) fn inner(&self) -> LevelFilter {
        self.0
    }
}

impl TryFrom<String> for LevelFilterSerde {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(LevelFilterSerde(
            LevelFilter::from_str(&value)
                .map_err(|e| format!("Couldn't parse log level filter: {}", e))?,
        ))
    }
}

impl Config {
    pub(crate) fn parse(cfg: &str) -> Result<Self, String> {
        toml::from_str::<Config>(cfg).map_err(|e| format!("TOML configuration parse error: {}", e))
    }

    pub(crate) fn make_commands(self, chains: &Pointers) -> Vec<Box<dyn Widget>> {
        self.commands.into_iter().map(|c| c.into_widget(&self.settings, chains)).collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilterSerde(LevelFilter::DEBUG),
                display: "0".parse().unwrap(),
                hide: "rshift+0".parse().ok(),
                show_console: false,
                indicators: Indicator::default_set(),
            },
            commands: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
struct FlagSpec {
    label: String,
    getter: fn(&Pointers) -> &Bitflag<u8>,
}

impl std::fmt::Debug for FlagSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlagSpec {{ label: {:?} }}", self.label)
    }
}

impl FlagSpec {
    fn new(label: &str, getter: fn(&Pointers) -> &Bitflag<u8>) -> FlagSpec {
        FlagSpec { label: label.to_string(), getter }
    }
}

impl TryFrom<String> for FlagSpec {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        macro_rules! flag_spec {
            ($x:expr, [ $( ($flag_name:ident, $flag_label:expr), )* ]) => {
                match $x {
                    $(stringify!($flag_name) => Ok(FlagSpec::new($flag_label, |c| &c.$flag_name)),)*
                    e => Err(format!("\"{}\" is not a valid flag specifier", e)),
                }
            }
        }
        flag_spec!(value.as_str(), [
            (render_world, "Render World"),
            (render_objects, "Render Objects"),
            (render_mobs, "Render Mobs"),
            (render_effects, "Render Effects"),
            (debug_render0, "Debug #0 (Low Col + Planes)"),
            (debug_render1, "Debug #1 (High Col)"),
            (debug_render2, "Debug #2 (Objects)"),
            (debug_render3, "Debug #3 (Low Col?)"),
            (debug_render4, "Debug #4 (Low Col?)"),
            (debug_render5, "Debug #5 (Walls?)"),
            (debug_render6, "Debug #6 (Wall Jump Col)"),
            (debug_render7, "Debug #7 (Edge/Cliff Col)"),
            (debug_show, "Debug Show"),
            (grapple_debug_path, "Grapple Debug (Path)"),
            (grapple_debug_col, "Grapple Debug (Col)"),
            (player_no_goods_consume, "No goods consume"),
            (player_no_resource_item_consume, "No resource consume"),
            (player_no_revival_consume, "No revival consume"),
            (player_hide, "Hide"),
            (player_silence, "Silence"),
            (player_no_dead, "No Dead"),
            (player_exterminate, "Exterminate"),
            (player_exterminate_stamina, "Exterminate Stamina"),
            (all_no_dead, "All No Dead"),
            (all_no_damage, "All No Damage"),
            (all_no_hit, "All No Hit"),
            (all_no_attack, "All No Attack"),
            (all_no_move, "All No Move"),
            (all_no_update_ai, "All No Update AI"),
            (all_no_stamina_consume, "All No Stamina Consume"),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_parse() {
        println!(
            "{:?}",
            toml::from_str::<toml::Value>(include_str!("../../jdsd_sekiro_practice_tool.toml"))
        );
        println!("{:?}", Config::parse(include_str!("../../jdsd_sekiro_practice_tool.toml")));
    }

    #[test]
    fn test_parse_errors() {
        println!(
            "{:#?}",
            Config::parse(
                r#"commands = [ { boh = 3 } ]
                [settings]
                log_level = "DEBUG"
                "#
            )
        );
    }
}
