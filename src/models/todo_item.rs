
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    #[serde(default)]
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
    pub fn read(id: u16) -> Result<Self, std::io::Error> {
        println!("read\r\n{:#?}", id);
        Ok(TodoItem {
            id,
            title: "a previously stored item".to_string(),
            description: "description".to_string(),
            checked: false,
            todo_list_id: 1,
        })
    }
    pub fn update(&mut self) -> Option<std::io::Error> {
        println!("create\r\n{:#?}", self);
        None
    }
    pub fn delete(id: u16) {
        println!("deleted: {:#?}", id);
        None
    }
    pub fn read_all_by_list_id(list_id: u16) -> Option<Vec<Self>> {
        Some(
            vec![
                TodoItem { id: 0, title: "ToDo 1".to_string(), description: "ToDo 1 description".to_string(), checked: true, todo_list_id: 1 },
                TodoItem { id: 1, title: "ToDo 2".to_string(), description: "ToDo 2 description".to_string(), checked: true, todo_list_id: 1 },
            ]
        )
    }

    pub fn delete_all_by_list_id(list_id: u16) {

    }

    pub fn is_exist(id: u16) -> bool {
        true
    }
}