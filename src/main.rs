mod db;
mod models;
mod commands;

use commands::{add::add_task, edit::change_task_date, list::list_tasks};
use db::setup_database;
use clap::{App, Arg, SubCommand};

fn main() {
    // Configura o banco de dados
    if let Err(e) = setup_database() {
        eprintln!("Failed to set up the database: {}", e);
        return;  
    }

    let matches = App::new("rustodo")
        .version("1.0")
        .author("Joaquim Chianca | github.com/joaquimchianca")
        .about("Your Task Management CLI")
        .subcommand(SubCommand::with_name("add")
            .about("Add a new task")
            .arg(Arg::with_name("name").help("Name of the task.").required(true))
            .arg(Arg::with_name("date").help("Due date of the task in YYYY-MM-DD format.").short('d').long("date").takes_value(true))
            .arg(Arg::with_name("priority").help("Priority of the task (1-4).").short('p').long("priority").takes_value(true))
        )
        .subcommand(SubCommand::with_name("ls")
            .about("List tasks")
            .arg(Arg::with_name("checked")
                .short('c')
                .long("checked")
                .help("List all completed tasks"))
            .arg(Arg::with_name("week")
                .short('w')
                .long("week")
                .help("List tasks for this week"))
            .arg(Arg::with_name("month")
                .short('m')
                .long("month")
                .help("List tasks for this month")))
        .subcommand(SubCommand::with_name("rm")
            .about("Remove a task")
            .arg(Arg::with_name("ids")
                .help("IDs of the tasks to remove, separated by commas")
                .takes_value(true)
                .required(true))
        )  
        .subcommand(SubCommand::with_name("ok")
            .about("Mark a task as done.")
            .arg(Arg::with_name("ids")
                .help("IDs od the tasks to be checked as done, separated by commas")
                .takes_value(true)
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("edit")
            .about("Change the task date through ID.")
            .arg(Arg::with_name("id")
                .help("The ID of the task to change date.")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("date")
                .short('d')
                .long("date")
                .help("New date for the task.")
                .takes_value(true)
                .required(true)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let name = matches.value_of("name").unwrap().to_string();
        let date = matches.value_of("date");
        let priority = matches.value_of("priority");

        match add_task(name, priority, date) {
            Ok(task_id) => println!("{}", task_id),
            Err(e) => println!("Failed to add task: {}", e)
        };
    } else if let Some(matches) = matches.subcommand_matches("ls") {
        let filter = if matches.is_present("checked") {
            "checked"
        } else if matches.is_present("week") {
            "week"
        } else if matches.is_present("month") {
            "month"
        } else {
            "default"
        };

        if let Err(e) = list_tasks(filter) {
            eprintln!("Error: {}", e);
        }
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        if let Some(ids) = matches.value_of("ids") {
            let id_list: Vec<&str> = ids.split(",").collect();
            match commands::remove::delete_tasks(id_list) {
                Ok(removed_ids) => println!("{:?}", removed_ids),
                Err(e) => eprintln!("Failed to remove tasks {}", e)
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("ok") {
        if let Some(ids) = matches.value_of("ids") {
            let id_list: Vec<&str> = ids.split(",").collect();
            match commands::done::check_task(id_list) {
                Ok(ids) => println!("{:?}", ids),  
                Err(e) => eprintln!("failed to check tasks {}", e)
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        let task_id = matches.value_of("id").unwrap();
        let new_date = matches.value_of("date").unwrap();
        match change_task_date(task_id, new_date) {
            Ok(_) => println!("Task {} remarked to {}", task_id, new_date),
            Err(e) => println!("Error: {}", e)
        }
    }    
}
