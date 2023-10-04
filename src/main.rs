use todo_app::Todo;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo_app <action> [item]");
        return;
    }

    let action = &args[1];

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        if args.len() < 3 {
            println!("Please provide an item to add.");
            return;
        }
        let item = &args[2];

        todo.insert(item.to_string());
        match todo.save() {
            Ok(_) => println!("Todo Saved"),
            Err(e) => println!("error occurred: {}", e),
        }
    } else if action == "complete" {
        if args.len() < 3 {
            println!("Please provide an item to mark as complete.");
            return;
        }
        let item = &args[2];
        match todo.complete(&item) {
            None => println!("{} is not present", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(e) => println!("an error occurred: {}", e),
            },
        }
    }else if action =="remove"{
        if args.len() < 3{
            println!("please provide an item to mark as complete");
            return;
        }
        let item = &args[2];
        todo.remove(item.to_string());
        match todo.save(){
            Ok(_) => println!("todo saved"),
            Err(e) => println!("an error occurred: {}", e)
        }
    } else if action == "list" {
        for (i, v) in todo.list().iter().enumerate() {
            println!("{}:{}", i + 1, v)
        }
    }
}
