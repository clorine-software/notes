use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tokio::fs;
use home::home_dir;
use std::collections::HashMap;
use std::str::FromStr;

use crate::var::*;
use crate::idsb32c6::*;

// data.ron struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    pub version: String,
    pub tasks: HashMap<TaskId, Task>,
    pub groups: HashMap<GroupId, Group>,
    pub task_order: Vec<TaskId>,
 	pub group_order: Vec<GroupId>,
}
impl Root {
    pub async fn add_task(&mut self, task: Task, parent: GroupId) -> Result<()> {
        let mut id = TaskId::random();
        while self.tasks.contains_key(&id) {
            id = TaskId::random();
        }
        self.tasks.insert(id.clone(), task);
        
        let parent_group = self.groups.get_mut(&parent).ok_or_else(|| anyhow!("No such group"))?;
        parent_group.contents_tasks.push(id.clone());

        self.task_order.push(id);
        
        Ok(())
    }
    pub async fn add_group(&mut self, group: Group, parent: GroupId) -> Result<()> {
        let mut id = GroupId::random();
        while self.groups.contains_key(&id) {
            id = GroupId::random();
        }
        self.groups.insert(id.clone(), group);

        if parent != GroupId::from_str("root").unwrap() {
            let parent_group = self.groups.get_mut(&parent).ok_or_else(|| anyhow!("No such group"))?;
            parent_group.contents_groups.push(id.clone());
        }

        self.group_order.push(id);
        
        Ok(())
    }
}

// task struct
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Task {
    pub name: String,
    pub content: Content,
}

// group struct
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub contents_tasks: Vec<TaskId>,
    pub contents_groups: Vec<GroupId>,
}

// member struct
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Content {
    pub text: String,
    pub files: Vec<String>, //paths
}

pub async fn get_data() -> Result<Root> {
    let mut data_dir = home_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
    data_dir.push(DATA_DIR);
    if !data_dir.exists() { fs::create_dir_all(&data_dir).await?; }
    let data_file = data_dir.join(DATA_FILE);
    if data_file.exists() {
        Ok(ron::from_str(&fs::read_to_string(data_file).await?)?)
    } else {
        Ok(Root{
            version: String::from(VERSION),
            tasks: HashMap::new(),
            groups: HashMap::new(),
            task_order: Vec::new(),
            group_order: Vec::new(),
        })
    }
}

pub async fn save_data(mut data: Root) -> Result<()> {
    data.version = String::from(VERSION);
    let mut data_dir = home_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
    data_dir.push(DATA_DIR);
    if !data_dir.exists() { fs::create_dir_all(&data_dir).await?; }
    let data_file = data_dir.join(DATA_FILE);
    fs::write(data_file, ron::ser::to_string_pretty(&data,
            ron::ser::PrettyConfig::new().depth_limit(4).indentor("  ".to_string())
            )?).await?;
    Ok(())
}
