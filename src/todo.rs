use crate::util::{use_input, use_integer_input};

#[derive(Debug)]
pub struct TodoItem {
    pub id: i32,
    pub name: String,
    pub completed: bool,
}
#[derive(Debug)]
pub struct TodoList {
    pub list: Vec<TodoItem>,
}




impl TodoList {
    fn reorder_tasks(&mut self) {
        for i in 0..self.list.len() {
            self.list[i].id = (i + 1) as i32;
        }
    }

    pub fn init(&mut self, num: i32) {
        match num {
            1 => self.create_task(),
            2 => self.remove_task(),
            3 => self.reset(),
            4 => self.print_tasks(),
            5 => self.mark_done(),
            _ => println!("Invalid choice")
        }
    }

    pub fn create_task(&mut self) {
        println!("chose a name for the task");
        let name = use_input();
        let new_task = TodoItem {
            id: (self.list.len() + 1) as i32,
            name,
            completed: false,
        };

        self.list.push(new_task);

        println!("\n");
        self.print_tasks();

    }

    pub fn print_tasks(&self) {
        let list = &self.list;
        let list_length: usize = list.len();

        println!("\n \n \n \n \n tasks: ");
        for i in 0..list_length {
            if list.len() == 0 {
                println!("there are no tasks");
                break
            }

            let is_completed = if list[i].completed { "completed" } else { "in progress" };
            println!("{}- '{}' {}", self.list[i].id, list[i].name, is_completed)
        }
        use_input();
    }

    pub fn remove_task(&mut self) {
        println!("Chose the number (index) of the task: ");
        let choice  = use_integer_input();
        let mut found = false;

        self.list.retain(|x| {
            if choice == x.id {
                found = true;
                false
            }else {
                true
            }
        });

        if !found {
            println!("task was not found");
        }else {
            self.reorder_tasks()
        }
    }

    pub fn mark_done(&mut self) {
        println!("Chose the number (index) of the task: ");
        let choice  = use_integer_input();
        let mut found = false;

        for i in 0..self.list.len() {
            if choice == self.list[i].id {
                found = true;
                self.list[i].completed = true;
            }
        }

        if !found {
            println!("task was not found");
        }
    }

    pub fn reset(&mut self) {
        self.list.clear();
    }
}


