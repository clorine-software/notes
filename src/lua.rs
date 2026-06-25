use home::home_dir;
use tokio::fs;
use mlua::Lua;

use crate::var::*;

// config.toml struct
// #[derive(Debug, Deserialize)]
// pub struct Config {
//     pub style: Option<Style>,
// }
// config.toml [styles] struct
// #[derive(Debug, Deserialize)]
// pub struct Style {
//     #[serde(default)]
//     pub before_text: String,
//     #[serde(default)]
//     pub after_text: String,
//     #[serde(default)]
//     pub before_specials: String,
//     #[serde(default)]
//     pub after_specials: String,
//     #[serde(default)]
//     pub before_commons: String,
//     #[serde(default)]
//     pub after_commons: String,
//     #[serde(default)]
//     pub before_special_unit: String,
//     #[serde(default)]
//     pub after_special_unit: String,
//     #[serde(default)]
//     pub before_common_unit: String,
//     #[serde(default)]
//     pub after_common_unit: String,
// }

pub async fn get_config() -> mlua::Result<String> {
    let mut data_dir = home_dir().ok_or_else(|| mlua::Error::external("Home directory not found"))?;
    data_dir.push(CONFIG_DIR);
    if !data_dir.exists() { fs::create_dir_all(&data_dir).await?; }
    let data_file = data_dir.join(CONFIG_FILE);
    if data_file.exists() {
        Ok(fs::read_to_string(data_file).await?)
    } else {
        fs::write(data_file, DEFAULT_CONFIGURATION).await?;
        Ok(String::from(DEFAULT_CONFIGURATION))
    }
}

pub async fn get_display_func() -> mlua::Result<(mlua::Lua, mlua::Function, mlua::Function)> {
    let script = get_config().await?;
    
    let lua = Lua::new();

    let value_table = lua.create_table()?;

    let colorize = lua.create_table()?;

    colorize.set("black", "\x1b[0;30m")?;
    colorize.set("red", "\x1b[0;31m")?;
    colorize.set("green", "\x1b[0;32m")?;
    colorize.set("yellow", "\x1b[0;33m")?;
    colorize.set("blue", "\x1b[0;34m")?;
    colorize.set("purple", "\x1b[0;35m")?;
    colorize.set("cyan", "\x1b[0;36m")?;
    colorize.set("white", "\x1b[0;37m")?;

    colorize.set("on_black", "\x1b[40m")?;
    colorize.set("on_red", "\x1b[41m")?;
    colorize.set("on_green", "\x1b[42m")?;
    colorize.set("on_yellow", "\x1b[43m")?;
    colorize.set("on_blue", "\x1b[44m")?;
    colorize.set("on_purple", "\x1b[45m")?;
    colorize.set("on_cyan", "\x1b[46m")?;
    colorize.set("on_white", "\x1b[47m")?;

    colorize.set("bold_black", "\x1b[1;30m")?;
    colorize.set("bold_red", "\x1b[1;31m")?;
    colorize.set("bold_green", "\x1b[1;32m")?;
    colorize.set("bold_yellow", "\x1b[1;33m")?;
    colorize.set("bold_blue", "\x1b[1;34m")?;
    colorize.set("bold_purple", "\x1b[1;35m")?;
    colorize.set("bold_cyan", "\x1b[1;36m")?;
    colorize.set("bold_white", "\x1b[1;37m")?;

    colorize.set("reset", "\x1b[0m")?;

    value_table.set("colorize", colorize)?;
    lua.globals().set("v", value_table)?;

    lua.load(script).exec()?;

    let result_group: mlua::Function = lua.globals().get("decorate_group")?;
    let result_task: mlua::Function = lua.globals().get("decorate_task")?;
    Ok((lua, result_group, result_task))
}
