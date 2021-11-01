
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    #[serde(skip_deserializing)]
    pub id: u16,
    pub title: String,
    pub description: String,
    pub checked: bool,
    pub todo_list_id: u16,
}

impl TodoItem {
    pub fn create(&mut self) -> Option<std::io::Error> {
        println!("create\r\n{:#?}", self);
        None
    }
    pub fn read(&mut self) -> Option<std::io::Error> {
        println!("create\r\n{:#?}", self);
        None
    }
    pub fn update(&mut self) -> Option<std::io::Error> {
        println!("create\r\n{:#?}", self);
        None
    }
    pub fn delete(&mut self) -> Option<std::io::Error> {
        println!("create\r\n{:#?}", self);
        None
    }
}