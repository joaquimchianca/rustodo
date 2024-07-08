#![allow(warnings)]

use crate::db;
use chrono::Local;
use colored::*;

pub fn list_tasks(filter: &str) -> Result<(), String> {
    match db::list_tasks_db(filter) {
        Ok(tasks) if tasks.is_empty() => {
            let message = match filter {
                "checked" => "Você ainda não completou nenhuma tarefa. Vamos ao trabalho!",
                "month" => "Você não possui tarefas cadastradas para esse mês. Curta as férias!",
                "week" => "Você não possui tarefas cadastradas para essa semana. Aproveite!",
                _ => "Você não possui tarefas cadastradas para fazer. Que bom!"
            };
            println!("{}", message);
            Ok(())
        },

        Ok(tasks) => {
            println!(); 
            println!("{:<15} {:<27} {:<16} {:<24}", "DATE", "TASK", "PRIORITY", "ID");
            println!("------------------------------------------------------------------------------------");
            for task in tasks {
                let today = Local::today().naive_local();

                let display_date = if task.date < today {
                    task.date.format("%Y-%m-%d").to_string().red()
                } else if task.date == today {
                    "Hoje".to_string().green()
                } else {
                    task.date.format("%Y-%m-%d").to_string().normal()
                };
                
                // trucamento para mensagem longa
                let truncated_task_name = if task.name.len() > 34 {
                    format!("{}...", &task.name[..31])
                } else {
                    task.name.clone()
                };

                // deixando colorido
                let display_priority = match task.priority {
                    1 => "High".bright_cyan(),
                    2 => "Medium".bright_blue(),
                    3 => "Low".bright_magenta(),
                    _ => "None".bright_black(),
                };

                if filter == "checked" {
                    println!("{:<15} {:<30} {:<13} {:<24}", 
                             display_date.green(), truncated_task_name.green(), display_priority.green(), task.id.green());
                } else {
                    println!("{:<15} {:<30} {:<13} {:<24}", 
                             display_date, truncated_task_name, display_priority, task.id);
                }
            }
            Ok(())
        },

        Err(e) => Err(format!("Failed to list tasks: {}", e)),
    }
}
