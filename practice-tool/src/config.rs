use std::str::FromStr;

use hudhook::tracing::metadata::LevelFilter;
use libsekiro::prelude::*;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::Widget;
use serde::Deserialize;

use crate::widgets::flag::flag_widget;
use crate::widgets::group::group;
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
            CfgCommand::NudgePosition { nudge, nudge_up, nudge_down } => {
                nudge_position(chains.position.clone(), nudge, nudge_up, nudge_down)
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
        flag_spec!(
            value.as_str(),
            [
                (render_world, "Render World"),
                (render_objects, "Render Objects"),
                (render_mobs, "Render Mobs"),
                (render_effects, "Render Effects"),
                (debug_render0, "Debug Render #0"),
                (debug_render1, "Debug Render #1"),
                (debug_render2, "Debug Render #2"),
                (debug_render3, "Debug Render #3"),
                (debug_render4, "Debug Render #4"),
                (debug_render5, "Debug Render #5"),
                (debug_render6, "Debug Render #6"),
                (debug_render7, "Debug Render #7"),
                (debug_render8, "Debug Render #8"),
                (player_no_goods_consume, "No goods consume"),
                (player_no_resource_item_consume, "No resource consume"),
                (player_no_revival_consume, "No revival consume"),
                (player_hide, "Hide"),
                (player_no_dead, "No Dead"),
                (all_no_dead, "All No Dead"),
                (all_no_damage, "All No Damage"),
                (all_no_hit, "All No Hit"),
                (all_no_attack, "All No Attack"),
                (all_no_move, "All No Move"),
                (all_no_update_ai, "All No Update AI"),
                (all_no_stamina_consume, "All No Stamina Consume"),
            ]
        )
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
