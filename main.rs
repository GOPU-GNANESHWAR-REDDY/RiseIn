#[derive(Clone)] // Derive the Clone trait for Task automatically
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }

    fn display_details(&self) {
        println!("Task ID: {}", self.id);
        println!("Task Description: {}", self.description);
        println!("Task Completed: {}", self.completed);
    }
}

struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    fn add_task(&mut self, description: String) -> Task {
        let id = self.tasks.len() + 1;
        let new_task = Task::new(id, description);
        self.tasks.push(new_task.clone()); // Cloning is now possible because Task implements Clone
        new_task
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        for task in &mut self.tasks {
            if task.id == id {
                task.complete();
                return Some(task);
            }
        }
        None
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            task.display_details();
        }
    }
}

fn main() {
    let mut todo_list = ToDoList { tasks: Vec::new() };
    let task1 = todo_list.add_task(String::from("Join in BootCamp"));
    let task2 = todo_list.add_task(String::from("Attend the Events regularly"));
    let task3 = todo_list.add_task(String::from("Participate in Mini Hackathon"));
    let task4 = todo_list.add_task(String::from("Thanks giving for this Opportunity"));

    todo_list.complete_task(task1.id);
    todo_list.complete_task(task2.id);
    todo_list.complete_task(task3.id);
    todo_list.complete_task(task4.id);

    todo_list.list_tasks();
}
