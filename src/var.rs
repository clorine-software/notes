
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DATA_DIR: &str = ".clorine/data/notes";
pub const CONFIG_DIR: &str = ".clorine/config/notes";

pub const DATA_FILE: &str = "data.ron";
pub const CONFIG_FILE: &str = "config.lua";

pub const DEFAULT_CONFIGURATION: &str = r#"
function decorate_task(iter, key, parent_key, name, content_text, content_files)
  return " " .. string.rep("  ", iter) .. "    " .. v.colorize.yellow .. key .. v.colorize.reset .. ". " .. name
end

function decorate_group(iter, key, name, description, is_empty) 
  local full_dir_symbol = " "
  local empty_dir_symbol = " "
  local dir_symbol
  if is_empty then dir_symbol = empty_dir_symbol else dir_symbol = full_dir_symbol end
  return " " .. string.rep("  ", iter) .. v.colorize.yellow .. dir_symbol .. key .. v.colorize.reset .. ". " .. name
end
"#;

