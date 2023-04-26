use std::str::FromStr;

use hudhook::tracing::metadata::LevelFilter;
use libsekiro::memedit::Bitflag;
use libsekiro::prelude::*;
use practice_tool_utils::widgets::Widget;
use practice_tool_utils::{get_key_code, KeyState};
use serde::Deserialize;

use crate::widgets::flag::Flag;
use crate::widgets::nudge_pos::NudgePosition;
use crate::widgets::position::SavePosition;
use crate::widgets::quitout::Quitout;
use crate::widgets::savefile_manager::SavefileManager;

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) settings: Settings,
    commands: Vec<CfgCommand>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub(crate) log_level: LevelFilterSerde,
    pub(crate) display: KeyState,
    #[serde(default)]
    pub(crate) dxgi_debug: bool,
    #[serde(default)]
    pub(crate) show_console: bool,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
#[serde(untagged)]
enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager")]
        hotkey_load: KeyState,
        hotkey_back: KeyState,
        hotkey_close: KeyState,
    },
    Flag {
        flag: FlagSpec,
        hotkey: Option<KeyState>,
    },
    Position {
        #[serde(rename = "position")]
        hotkey: KeyState,
        modifier: KeyState,
    },
    NudgePosition {
        nudge: f32,
        nudge_up: Option<KeyState>,
        nudge_down: Option<KeyState>,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: KeyState,
    },
}

#[derive(Deserialize, Debug)]
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
        let de = toml::de::Deserializer::new(cfg);
        serde_path_to_error::deserialize(de)
            .map_err(|e| format!("TOML config error at {}: {}", e.path(), e.inner()))
    }

    fn make_command(cmd: &CfgCommand, chains: &Pointers) -> Box<dyn Widget> {
        let widget = match cmd {
            CfgCommand::Flag { flag, hotkey } => {
                Box::new(Flag::new(&flag.label, (flag.getter)(chains).clone(), *hotkey))
                    as Box<dyn Widget>
            },
            CfgCommand::SavefileManager { hotkey_load, hotkey_back, hotkey_close } => {
                SavefileManager::new_widget(*hotkey_load, *hotkey_back, *hotkey_close)
            },
            CfgCommand::Position { hotkey, modifier } => {
                Box::new(SavePosition::new(chains.position.clone(), *hotkey, *modifier))
            },
            CfgCommand::NudgePosition { nudge, nudge_up, nudge_down } => Box::new(
                NudgePosition::new(chains.position.clone(), *nudge, *nudge_up, *nudge_down),
            ),
            // CfgCommand::CycleSpeed { cycle_speed, hotkey } => Box::new(CycleSpeed::new(
            //     cycle_speed,
            //     [chains.animation_speed.clone(), chains.torrent_animation_speed.clone()],
            //     *hotkey,
            // )),
            CfgCommand::Quitout { hotkey } => {
                Box::new(Quitout::new(chains.quitout.clone(), *hotkey))
            },
        };

        widget
    }

    pub(crate) fn make_commands(&self, chains: &Pointers) -> Vec<Box<dyn Widget>> {
        self.commands.iter().map(|cmd| Config::make_command(cmd, chains)).collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilterSerde(LevelFilter::DEBUG),
                display: KeyState::new(get_key_code("0").unwrap()),
                dxgi_debug: false,
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
        flag_spec!(value.as_str(), [
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
