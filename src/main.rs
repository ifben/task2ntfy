use task_hookrs::task::Task;
use chrono::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use clap::Parser;

pub type Tasks = Vec<Task>;

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

    /// base URL if you are self-hosting
    #[arg(short, long, default_value = "https://ntfy.sh/")]
    base_url: String,

    /// the earliest hour you want to be notified, in 24 hour time
    #[arg(short, long, default_value_t = 9)]
    earliest: u32,

    /// how often to check upcoming tasks, in seconds
    #[arg(short, long, default_value_t = 60)]
    check_every: u32,

    /// run continuously, or run once
    #[arg(short, long, default_value_t = false)]
    once: bool,

    /// how soon you want to be notified of upcoming event, in hours
    #[arg(short, long, default_value_t = 24)]
    within: u32,
}
fn main() -> Result<(), ureq::Error> {

    let args = Args::parse();
    
    let interval = Duration::from_secs(args.check_every as u64); // how often the loop will run
    let mut next_time = Instant::now() + interval; // set the time for the next loop to run
    let mut url = "https://ntfy.sh/".to_string(); // use the ntfy.sh instance as default
    if args.base_url.is_empty() == false { url = args.base_url; } // if an instance was provided, use it
    let path = url + &args.subscription; // build the absolute path to the ntfy instance

    let mut message: Vec<String> = Vec::new(); // vector that tracks what messages need to be sent to ntfy
    let mut uuids: Vec<String> = Vec::new(); // vector that tracks the UUID of already sent messages to prevent multiple notifications

    loop {  

        message.clear(); // clear the vector tracking what messages to send each loop

        let mut task = std::process::Command::new("task");
       
            task.arg("rc.json.array=on")
                    .arg("rc.confirmation=off")
                    .arg("rc.json.depends.array=on")
                    .arg("rc.color=off")
                    .arg("rc._forcecolor=off")
                    .arg("status:pending");
            task.arg("export");                         // output simplified JSON of all pending tasks
    
            if let Ok(output) = task.output() { // if there are pending tasks, proceed
                let data = String::from_utf8_lossy(&output.stdout);
                let pending_tasks: Tasks = serde_json::from_str(&data).unwrap(); // use serde to parse JSON
                for task in pending_tasks {
                   
                    if let Some(_i) = task.due() { // need to use an if let here since due() returns an option (tasks don't require due dates), even though ours only ever will here

                        let tw_due = task.due().unwrap().to_string(); // unwrap taskwarrior's due date
                        let tw_parsed = NaiveDateTime::parse_from_str(&tw_due, "%Y-%m-%d%H:%M:%S").unwrap(); // parse taskwarrior due date to something comparable
                        
                        let now: DateTime<Local> = Local::now(); // get current time
                        let notification_time = NaiveTime::from_hms_opt(args.earliest.into(), 0, 0).unwrap(); // parse the earliest time we want to be notified
    
                        let difference = tw_parsed - now.naive_local(); // calculate if we are within notification time 

                        if difference.num_hours() < args.within as i64 && difference.num_hours() > 0 && now.time() > notification_time { // if we're within the notification range and it's not too early, proceed
                            if uuids.len() > 0 { // if we've already added a UUID to our vector, we need to check for matches
                                'last: for _i in 0..uuids.len() {
                                        for j in 0..uuids.len() { // this nested for loop sucks
                                            if task.uuid().to_string() == uuids[j] {
                                                break 'last; // break loop if we find the UUID already sent
                                            }
                                        }
                                        // if nothing matched in there, push it
                                        message.push(task.description().to_string());
                                        uuids.push(task.uuid().to_string());
                                }
                            } else {
                              // if the UUID vector has no length, we haven't sent anything yet, push it
                              message.push(task.description().to_string());
                              uuids.push(task.uuid().to_string());
                            }
                            
                        }    
                   }
                }
            }

        for i in 0..message.len() {
            // loop through the message vector to send all pending notifications to ntfy
            let _resp = ureq::post(&path).send_string(&message[i])?;
        }
        sleep(next_time - Instant::now());
        next_time += interval;
        println!("Looping...");

    }       
}