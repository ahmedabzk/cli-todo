use clap::{Parser, Args, Subcommand};

use todo_app::Todo;

#[derive(Debug, Parser)]
pub struct Cli{
    #[clap(subcommand)]
    pub action: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command{
    Add(Item),
    Complete(Item),
    Remove(Item),
    List,
}

#[derive(Debug, Args)]
pub struct Item{
    pub name: String,
}

pub fn todo_list() -> Result<(), std::io::Error>{
    let mut todo = Todo::new().expect("Initialisation of db failed");
    let cli = Cli::parse();

    match cli.action{
        Command::Add(item) =>{
            todo.insert(item.name);
            todo.save()?;
        }
        Command::Complete(item) =>{
            todo.complete(&item.name);
            todo.save()?;
        }
        Command::Remove(item) =>{
            todo.remove(item.name);
            todo.save()?;
        }
        Command::List =>{
            for (i, items) in todo.list().iter().enumerate(){
                println!("{}:{}", i+1, items)
            }
        }
    }
    Ok(())
}

