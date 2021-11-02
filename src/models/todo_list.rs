use serde::{Deserialize, Serialize};
use crate::models::todo_item::TodoItem;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    #[serde(default)]
    pub id: u16,
    pub title: String,
}

impl TodoList {
    pub fn create(&mut self) -> Option<std::io::Error> {
        println!("create\r\n{:#?}", self);
        None
    }
    pub fn read(id: u16) -> Result<Self, std::io::Error> {
        println!("read\r\n{:#?}", id);
        Ok(TodoList {
            id,
            title: "a previously stored list".to_string(),
        })
    }
    pub fn update(&mut self) -> Option<std::io::Error> {
        println!("update: \r\n{:#?}", self);
        None
    }
    pub fn delete(id: u16) {
        println!("deleted: {:#?}", id);
    }

    pub fn is_exist(id: u16) -> bool {
        true
    }
}