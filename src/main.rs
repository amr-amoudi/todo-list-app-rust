use crate::util::use_integer_input;

mod todo;
mod util;

fn main() {
    let mut todo_list = todo::TodoList {
        list: Vec::new()
    };

    loop {
        println!("\n");
        println!("chose an option:");
        println!("1- add task");
        println!("2- delete task");
        println!("3- reset list");
        println!("4- show tasks");
        println!("5- mark as done");
        println!("6 - exit");

        let choice = use_integer_input();

        if choice == 6 {
            break;
        }

        todo_list.init(choice);
    }
}
