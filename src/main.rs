use task_hookrs::task::Task;
use chrono::prelude::*;
use std::time::Duration;
//use tokio::{task, time};
use clap::Parser;
//use std::future::Future;

pub type Tasks = Vec<Task>;


const HOURS: i64 = 24;
const CHECK_EVERY: u64 = 10;
//const EARLY: i64 = 9;

#[derive(Parser, Debug)]
#[command(name = "task2ntfy")]
#[command(author = "ben wirth <benwirth@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Push upcoming taskwarrior tasks to ntfy.sh subscription", long_about = None)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the name of your ntfy.sh subscription (ntfy.sh/<subscription>)
    #[arg(short, long, required = true)]
    subscription: String,

    /// the earliest hour you want to be notified, in 24 hour time
    #[arg(short, long, default_value_t = 9)]
    earliest: u8,

    /// how often to check upcoming tasks, in seconds
    #[arg(short, long, default_value_t = 60)]
    check_every: u8,

    /// run continously, or run once
    #[arg(short, long, default_value_t = false)]
    once: bool,

    /// how soon you want to be notified of upcoming event, in hours
    #[arg(short, long, default_value_t = 24)]
    within: u8,
}


//#[tokio::main]
fn main() {

    let args = Args::parse();

    println!("{}", args.subscription);

   
}

fn check_tasks() -> String {
    // this will probably need to return a vec of notifications to send rather than just a string
    // and we'll also need to take some input to replace HOURS const
    //let early = earliest as i64;
    let mut task = std::process::Command::new("task");

        task.arg("rc.json.array=on")
                .arg("rc.confirmation=off")
                .arg("rc.json.depends.array=on")
                .arg("rc.color=off")
                .arg("rc._forcecolor=off")
                .arg("status:pending");
        task.arg("export");
    
        let mut message = String::new();

        if let Ok(output) = task.output() {
            let data = String::from_utf8_lossy(&output.stdout);
            let pending_tasks: Tasks = serde_json::from_str(&data).unwrap();
            for task in pending_tasks {
                if let Some(_i) = task.due() {
                    //println!("Task due: {:?}", task.due().unwrap());
                    
                    let due = task.due().unwrap().to_string();
                    let now: DateTime<Local> = Local::now();

                    let parsed = NaiveDateTime::parse_from_str(&due, "%Y-%m-%d%H:%M:%S");

                    let naive = now.naive_local().to_string();
                    let naive_parsed = NaiveDateTime::parse_from_str(&naive, "%Y-%m-%d%H:%M:%S%.f");

                    let difference = parsed.unwrap() - naive_parsed.unwrap();
                    if difference.num_hours() < HOURS && difference.num_hours() > 0 {
                        message = task.description().to_string();
                    } else {
                        message = "Don't send notification!".to_string();
                    }

               }
            }
        }
        return message;
}