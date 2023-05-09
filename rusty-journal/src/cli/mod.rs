pub mod args;
pub mod tasks;

use std::io::Result;
use std::path::PathBuf;

fn main()
{
    println!("I'm only here to make sure the mod is loaded.");
}

pub fn add_task(journal_path: PathBuf, task: tasks::Task)
{
    todo!("Make the add task function :)");
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()>
{
    todo!("Make the complete task function :)");
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()>
{
    todo!("Make the list tasks function :)");
}