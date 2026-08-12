use anyhow::Result;
use colored::Colorize;
use dialoguer::Confirm;
use serde::{Deserialize, Serialize};

use crate::fsf;

// data.ron struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    pub special: Vec<Task>,
    pub common: Vec<Task>,
}

// task struct
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Task {
    pub name: String,
    pub content: Option<String>,
}

// config.toml struct
#[derive(Debug, Deserialize)]
pub struct Config {
    pub style: Option<Style>,
}

// config.toml [styles] struct
#[derive(Debug, Deserialize)]
pub struct Style {
    #[serde(default)]
    pub before_text: String,

    #[serde(default)]
    pub after_text: String,

    #[serde(default)]
    pub before_specials: String,

    #[serde(default)]
    pub after_specials: String,

    #[serde(default)]
    pub before_commons: String,

    #[serde(default)]
    pub after_commons: String,

    #[serde(default)]
    pub before_special_unit: String,

    #[serde(default)]
    pub after_special_unit: String,

    #[serde(default)]
    pub before_common_unit: String,

    #[serde(default)]
    pub after_common_unit: String,
}

pub async fn execute_list() -> Result<()> {
    let data = fsf::get_data().await?;

    for (i, v) in data.special.iter().enumerate() {
        println!(
            "{}. {}",
            i.to_string().yellow(),
            format!("! {}", v.name).red()
        );
    }
    for (i, v) in data.common.iter().enumerate() {
        println!("{}. - {}", i.to_string().yellow(), v.name);
    }

    Ok(())
}

pub async fn execute_display() -> Result<()> {
    let data = fsf::get_data().await?;
    let conf = fsf::get_config().await?;

    if let Some(style) = conf.style {
        print!("{}", style.before_text);

        print!("{}", style.before_specials);
        for (i, v) in data.special.iter().enumerate() {
            println!(
                "{}{}. {}{}",
                style.before_special_unit,
                i.to_string().yellow(),
                format!("! {}", v.name).red(),
                style.after_special_unit
            );
        }
        print!("{}", style.after_specials);

        print!("{}", style.before_commons);
        for (i, v) in data.common.iter().enumerate() {
            println!(
                "{}{}. - {}{}",
                style.before_common_unit,
                i.to_string().yellow(),
                v.name,
                style.after_common_unit
            );
        }
        print!("{}", style.after_commons);

        print!("{}", style.after_text);
    } else {
        for (i, v) in data.special.iter().enumerate() {
            println!(
                "{}. {}",
                i.to_string().yellow(),
                format!("! {}", v.name).red()
            );
        }
        for (i, v) in data.common.iter().enumerate() {
            println!("{}. - {}", i.to_string().yellow(), v.name);
        }
    }

    Ok(())
}

pub async fn execute_cat(index: usize, is_special: bool) -> Result<()> {
    let data = fsf::get_data().await?;

    match is_special {
        true => {
            println!(
                "{}. {}:",
                index.to_string().yellow(),
                format!("! {}", data.special[index].name).red()
            );
            if let Some(content) = &data.special[index].content {
                println!("{}", content);
            }
        }
        false => {
            println!(
                "{}. - {}:",
                index.to_string().yellow(),
                format!("{}", data.common[index].name).yellow()
            );
            if let Some(content) = &data.common[index].content {
                println!("{}", content);
            }
        }
    }

    Ok(())
}

pub async fn execute_new(name: String, content: Option<String>, is_special: bool) -> Result<()> {
    let mut data = fsf::get_data().await?;

    match is_special {
        true => {
            data.special.push(Task {
                name: name,
                content: content,
            });
        }
        false => {
            data.common.push(Task {
                name: name,
                content: content,
            });
        }
    }

    let _ = fsf::save_data(data).await?;

    Ok(())
}

pub async fn execute_delete(index: usize, is_special: bool, is_force: bool) -> Result<()> {
    let mut data = fsf::get_data().await?;

    match is_special {
        true => {
            if is_force == true {
                let _ = data.special.remove(index);
            } else {
                println!("Задача: {}", data.special[index].name.red());
                if Confirm::new().with_prompt("Удалить?").interact()? {
                    let _ = data.special.remove(index);
                }
            }
        }
        false => {
            if is_force == true {
                let _ = data.common.remove(index);
            } else {
                println!("Задача: {}", data.common[index].name.yellow());
                if Confirm::new().with_prompt("Удалить?").interact()? {
                    let _ = data.common.remove(index);
                }
            }
        }
    }

    let _ = fsf::save_data(data).await?;

    Ok(())
}

pub async fn execute_printjson() -> Result<()> {
    let data = fsf::get_data().await?;

    println!("{}", serde_json::to_string(&data)?);

    Ok(())
}
