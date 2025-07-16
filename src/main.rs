mod storage;
mod task;

use crate::storage::TaskStorage;
use crate::task::Task;
use std::io;
fn main() {
    let mut storage = TaskStorage::new();
    let mut next_id: u32 = 1;
    loop {
        println!(
            "\n1. Add task \n2. view Tasks \n3. Mark task as completed\n4. Delete Task\n5. Exit"
        );
        //parse return Result it return Ok or error
        //unwrap_or by using it return if there error or default value 0 is this case will not cause a panic if for exmpale choice is not a number
        let choice: u32 = read_input("Enter a choice :").trim().parse().unwrap_or(0);
        if choice == 1 {
            let title = read_input("Enter the title :");
            let desc = read_input("Enter the description");
            let new_task = Task::new(next_id, title, desc);
            storage.add(new_task);
            println!("Task is added with ID: {} ", next_id);
            next_id += 1;
        } else if choice == 2 {
            storage.view_all();
        } else if choice == 3 {
            let id: u32 = read_input("Enter the ID : ").trim().parse().unwrap_or(0);
            match storage.mark_compeleted(id) {
                Ok(_) => println!("Task marked as completed!"),
                Err(e) => println!("Error : {}", e),
            }
        } else if choice == 4 {
            let id: u32 = read_input("Enter the ID : ").trim().parse().unwrap_or(0);
            match storage.delete(id) {
                Ok(_) => println!("Task deleted!"),
                Err(e) => println!("Error : {}", e),
            }
        } else if choice == 5 {
            break;
        } else {
            println!("Invalid choice!");
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read.");
    return input.trim().to_string();
}
