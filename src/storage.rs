use crate::task::Task;
use std::collections::HashMap; // folder is crate
pub struct TaskStorage {
    tasks: HashMap<u32, Task>,
}

impl TaskStorage {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
    pub fn add(&mut self, new_task: Task) {
        self.tasks.insert(new_task.id, new_task);
    }
    pub fn view_all(&self) {
        if self.tasks.is_empty() {
            println!("No tasks!");
        } else {
            for task in self.tasks.values() {
                task.print();
            }
        }
    }
    //Result<(),String> () no value to return and String error
    pub fn mark_compeleted(&mut self, id: u32) -> Result<(), String> {
        //get it return immutabele cant modifiy to modifiy use get_mut and return option enum
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.mark_completed();
                Ok(())
                //empty we dont pass any value
            }
            None => Err("Task is not found".to_string()),
        }
    }
    pub fn delete(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.remove(&id) {
            Some(_) => Ok(()),
            None => Err("task is not found".to_string()),
        }
    }
}
