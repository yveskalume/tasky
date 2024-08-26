use std::io::{self, Write};

struct Task {
    id: u32,
    name: String,
    description: String,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn start() -> Self {
        TaskManager {
            tasks: Vec::new(),
        }
    }

    fn add_task(&mut self, task_name: String, task_description: String) {
        let id = self.tasks.len() as u32;
        self.tasks.push(
            Task {
                id,
                name: task_name,
                description: task_description,
            }
        )
    }

    fn list_tasks(&self) {
        for task in self.tasks.iter() {
            println!("  [{}] {} - {}", task.id + 1, task.name, task.description);
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::start();

    loop {
        println!("\nWelcom to Tasky");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("q. Quit");

        print!("Choose an option: ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter task Name: ");
                io::stdout().flush().unwrap();

                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                print!("Enter task description: ");
                io::stdout().flush().unwrap();

                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = description.trim().to_string();

                task_manager.add_task(name, description);
            }
            "2" => {
                task_manager.list_tasks()
            }

            "q" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
