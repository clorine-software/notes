use std::str::FromStr;

use clap::{Parser, Subcommand};
use anyhow::Result;
// use colored::Colorize;
// use dialoguer::Confirm;

use crate::data::*;
use crate::lua::*;
use crate::idsb32c6::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Display your notes without styles
    // List,
    ///Display your notes with styles
    // Display,
    /// Display note with content by index
    // Cat { 
        // index: usize,
        
        // #[arg(short, long)]
        // special: bool
    // },
    /// Create new note
    // New { 
        // name: String,
        // content: Option<String>,

        // #[arg(short, long)]
        // special: bool,
    // },
    /// Delete note by index
    // Delete {
        // index: usize,
        
        // #[arg(short, long)]
        // special: bool,

        // #[arg(short, long)]
        // force: bool,
    // },
    /// You can use this to create your own interface for CL.NET Notes
    #[command(aliases = &["list", "ls"])]
    List {
        #[arg(short, long)]
        groups_only: bool,
    },
    #[command(aliases = &["display", "ds"])]
    Display {
        #[arg(short, long)]
        groups_only: bool,
    },
    #[command(aliases = &["create", "new", "n", "mktask", "touch"])]
    Create {
        name: String,
        
        group_parent: GroupId,
    },
    #[command(aliases = &["creategroup", "newgroup", "ng", "mkgroup", "mkdir"])]
    CreateGroup {
        name: String,
        
        #[arg(short, long)]
        group_parent: Option<GroupId>,
    },
    // Cat {}
    // Delete {}
    // Move {}
    // Edit {}
    PrintJson,
}

pub async fn parse() -> Result<()> {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::List { groups_only } => {
            list(groups_only, false).await?;
        },
        Commands::Display { groups_only } => {
            list(groups_only, true).await?;
        },
        Commands::Create { name, group_parent } => {
            let mut data = get_data().await?;
            
            let task = Task {
                name: name,
                content: Content {
                    text: String::new(),
                    files: Vec::new(),
                },
            };

            data.add_task(task, group_parent).await?;

            let _ = save_data(data).await?;
        },
        Commands::CreateGroup { name, group_parent } => {
            let mut data = get_data().await?;
            
            let group = Group {
                name: name,
                description: String::new(),
                contents_tasks: Vec::new(),
                contents_groups: Vec::new(),
            };

            data.add_group(group, group_parent.unwrap_or_else(|| GroupId::from_str("root").unwrap())).await?;

            let _ = save_data(data).await?;
        },
        
    //     Commands::List => {
    //         let data = get_data().await?;

    //         for (i, v) in data.special.iter().enumerate() {
    //             println!("{}. {}", i.to_string().yellow(), format!("! {}", v.name).red());
    //         }
    //         for (i, v) in data.common.iter().enumerate() {
    //             println!("{}. - {}", i.to_string().yellow(), v.name);
    //         }
    //     },
    //     Commands::Display => {
    //         let data = get_data().await?;

    //         let conf = get_config().await?;

    //         if let Some(style) = conf.style {
    //             print!("{}", style.before_text);
                
    //             print!("{}", style.before_specials);
    //             for (i, v) in data.special.iter().enumerate() {
    //                 println!("{}{}. {}{}", style.before_special_unit,
    //                     i.to_string().yellow(),
    //                     format!("! {}", v.name).red(),
    //                     style.after_special_unit
    //                 );
    //             }
    //             print!("{}", style.after_specials);

    //             print!("{}", style.before_commons);
    //             for (i, v) in data.common.iter().enumerate() {
    //                 println!("{}{}. - {}{}", style.before_common_unit,
    //                     i.to_string().yellow(),
    //                     v.name,
    //                     style.after_common_unit
    //                 );
    //             }
    //             print!("{}", style.after_commons);
                
    //             print!("{}", style.after_text);
    //         } else {
    //             for (i, v) in data.special.iter().enumerate() {
    //                 println!("{}. {}", i.to_string().yellow(), format!("! {}", v.name).red());
    //             }
    //             for (i, v) in data.common.iter().enumerate() {
    //                 println!("{}. - {}", i.to_string().yellow(), v.name);
    //             }
    //         }
    //     },
    //     Commands::Cat { index, special } => {
    //         let data = get_data().await?;

    //         match special {
    //             true => {
    //                 println!("{}. {}:", index.to_string().yellow(),
    //                     format!("! {}", data.special[index].name).red()
    //                 );
    //                 if let Some(content) = &data.special[index].content { println!("{}", content);}
    //             },
    //             false => {
    //                 println!("{}. - {}:", index.to_string().yellow(),
    //                     format!("{}", data.common[index].name).yellow()
    //                 );
    //                 if let Some(content) = &data.common[index].content { println!("{}", content);}
    //             }
    //         }
    //     },
    //     Commands::New { name, content, special } => {
    //         let mut data = get_data().await?;

    //         match special {
    //             true => {
    //                 data.special.push(Task {name: name, content: content});
    //             },
    //             false => {
    //                 data.common.push(Task {name: name, content: content});
    //             }
    //         }

    //         let _ = save_data(data).await?;
    //     },
    //     Commands::Delete { index, special, force } => {
    //         let mut data = get_data().await?;

    //         match special {
    //             true => {
    //                 if force == true { let _ = data.special.remove(index); } else {
    //                     println!("Задача: {}", data.special[index].name.red());
    //                     if Confirm::new().with_prompt("Удалить?").interact()? {
    //                         let _ = data.special.remove(index);
    //                     }
    //                 }
    //             },
    //             false => {
    //                 if force == true { let _ = data.common.remove(index); } else {
    //                     println!("Задача: {}", data.common[index].name.yellow());
    //                     if Confirm::new().with_prompt("Удалить?").interact()? {
    //                         let _ = data.common.remove(index);
    //                     }
    //                 }
    //             }
    //         }

    //         let _ = save_data(data).await?;
    //     },
        Commands::PrintJson => {
            let data = get_data().await?;

            println!("{}", serde_json::to_string(&data)?);
        }
    }

    Ok(())
}

fn recursive_list_print(iter: usize, only_groups: bool, data: &Root, group_key: &GroupId) {
    println!(" {}{} {} - group", "  ".repeat(iter), group_key, data.groups[&group_key].name);

    for sub_id in &data.groups[&group_key].contents_groups {
        recursive_list_print(iter+1, only_groups, data, sub_id);
    }

    if !only_groups {
        for task_key in &data.groups[&group_key].contents_tasks {
            println!("   {}{} {} - task", "  ".repeat(iter), task_key, data.tasks[&task_key].name);
        }
    }
}

fn recursive_list_display(iter: usize, only_groups: bool, data: &Root, group_key: &GroupId, display_group_func: &mlua::Function, display_task_func: &mlua::Function) {
    {let text_to_display: String = display_group_func.call(
        (
            iter,
            group_key.clone().as_str(),
            data.groups[&group_key].name.clone(),
            data.groups[&group_key].description.clone(),
            data.groups[&group_key].contents_tasks.is_empty() && data.groups[&group_key].contents_groups.is_empty(), 
        )
    ).unwrap();
    println!("{}", text_to_display);}

    for sub_id in &data.groups[&group_key].contents_groups {
        recursive_list_display(iter+1, only_groups, data, sub_id, display_group_func, display_task_func);
    }

    if !only_groups {
        for task_key in &data.groups[&group_key].contents_tasks {
            let text_to_display: String = display_task_func.call(
                (
                    iter,
                    task_key.clone().as_str(),
                    group_key.clone().as_str(),
                    data.tasks[&task_key].name.clone(),
                    data.tasks[&task_key].content.text.clone(),
                    data.tasks[&task_key].content.files.clone(),
                )
            ).unwrap();
            println!("{}", text_to_display);
        }
    }
}

async fn list(groups_only: bool, display: bool) -> Result<()> {
    let data = get_data().await?;

    match groups_only {
        true => {
            for group_id in &data.group_order {
                let mut is_root_child = true;
                for group_sub_id in &data.group_order {
                    if data.groups[&group_sub_id].contents_groups.contains(group_id) {
                        is_root_child = false;
                    }
                }
                if is_root_child == true {
                    match display {
                        true => {
                            let (_lua, display_group_func, display_task_func) = get_display_func().await.map_err(|e| anyhow::anyhow!("{}", e))?;
                            recursive_list_display(0, true, &data, group_id, &display_group_func, &display_task_func)
                        },
                        false => {recursive_list_print(0, true, &data, group_id)},
                    }
                }
            }
        },
        false => {
            for group_id in &data.group_order {
                let mut is_root_child = true;
                for group_sub_id in &data.group_order {
                    if data.groups[&group_sub_id].contents_groups.contains(group_id) {
                        is_root_child = false;
                    }
                }
                if is_root_child == true {
                    match display {
                        true => {
                            let (_lua, display_group_func, display_task_func) = get_display_func().await.map_err(|e| anyhow::anyhow!("{}", e))?;
                            recursive_list_display(0, false, &data, group_id, &display_group_func, &display_task_func)
                        },
                        false => {recursive_list_print(0, false, &data, group_id)},
                    }
                }
            }
        }
    }

    Ok(())
}

