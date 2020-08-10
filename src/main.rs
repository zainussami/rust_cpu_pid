use psutil::process::Process;
use std::thread;
use std::time::Duration;
use chrono::prelude::*;
use std::process;
use std::io;
use std::io::Write;
use rusqlite::{params, Connection, Result};
use rusqlite::NO_PARAMS;
mod parser;

struct PidUtil {
    pid: u32,
    name: String,
    util: f64,
    timestamp: String,
}

const SAMPLES:u32 = 60;    // Number of Samples 
const INTERVAL:u64 = 1000;    // Sampling Interval in Miliseconds    

fn delay(millis: u64) {
    let timeout = Duration::from_millis(millis);
    thread::sleep(timeout);
}

fn display_welcome() {
        //Display Message 
        println!("RUST Process CPU utilization");
        println!("Please Enter Process IDs seperated by commas 
        e.g. 646,456,345 (Leave Blank to Monitor this application)");

}

//Funtion Handle Errors in PID inputs (Valid Input is u32) 
fn parse_pids(reader: String) -> Vec<u32>{
    let split_string = reader.split(',');
    let (numbers, errors): (Vec<_>, Vec<_>) = split_string
        .into_iter()
        .map(|s| s.parse::<u32>())
        .partition(Result::is_ok);
        let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap)
        .collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err)
        .collect();
        println!("Errors: {:?}", errors);
    numbers
}

fn collect_pid_data(pid : &u32)-> Result<()>{
    //Error Handling Check if the process Exists
    let pid_proc = Process::new(*pid);
    match pid_proc {
        Ok(mut pid_acquired) => {
            println!("Collecting CPU utilization Data for {:.100}", 
            pid_acquired.name().unwrap());      
            let mut counter = 0;
            //Access Database
            let conn = Connection::open("pid_data.db")?;
            while counter < SAMPLES { //Collect Data for 60 Seconds  
                delay(INTERVAL);
                 
                let local: DateTime<Local> = Local::now();  
                let cpu_util = pid_acquired.cpu_percent().unwrap() as f64; 
                print!(". ");
                io::stdout().flush().unwrap();  
                //Write to SQLite DB 
                let me = PidUtil {
                    pid: *pid,
                    name: pid_acquired.name().unwrap(),
                    timestamp: local.format("%Y-%m-%d %H:%M:%S").to_string(),
                    util: cpu_util,
                };
                conn.execute(
                    "INSERT INTO pid_util (pid, name, util, timestamp)
                              VALUES (?1, ?2, ?3, ?4)",
                    params![me.pid, me.name, me.util, me.timestamp],
                )?;
                counter +=1;
            }
            println!("\nSuccesfully wrote to PID {} CPU Utilization 
            Data to Database.", pid);
        },
        //Thread Fails But does not Panic and Reports an error
        Err(error) => println!("{:?}", error),
    };
    Ok(())
}

fn check_database() {
    let conn = Connection::open("pid_data.db");
    match conn {
        Ok(conn_acquired) =>{println!("{:?}", conn_acquired);
        let table =conn_acquired.execute(
            "create table if not exists pid_util (
                 pid integer,
                 name text not null,
                 util text,
                 timestamp text
             )",
            NO_PARAMS,
        );
        match table{
            Ok(table) =>println!("{:?}", table),
            Err(error) => panic!("{:?}", error),
        };    
    },
        Err(error) => panic!("{:?}", error),
    };
}

fn main() {
    
    let (mode,pid_lists)= parser::console_parser();
    
    //Check Database Status
    check_database();

    //Check the Mode
    if mode == true{  //Interactive Mode
        //Clear the Screen
        print!("{}[2J", 27 as char);
        loop{
            display_welcome();
            //Get User Input For PID
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            if input_text.len()<2 {
                println!("My pid is {}", process::id());
                input_text = process::id().to_string();
            }
            //Remove Newline Character
            let new_input = input_text.replace("\n", "");
            //Convert PID List from Strings
            let vec = parse_pids(new_input);        
            println!("Valid PIDs {:?} :", vec);
            //Spawn Multiple Threads
            let mut threads = Vec::new();
            for thread_no in 0..vec.len() {
                let int = vec[thread_no];
                threads.push(thread::spawn(move || {
                    let test = collect_pid_data(&int);
                    match test{
                        Ok(()) =>println!(""),
                        Err(error) => panic!("{:?}", error),
                    };   
                }))
            }
            for t in threads {
                t.join().unwrap();
            }            
        }
    }
    else{//Batch Mode
        //Convert PID List from Strings
        let vec = parse_pids(pid_lists);        
        println!("Valid PIDs {:?} :", vec);
        //Spawn Multiple Threads
        let mut threads = Vec::new();
        for thread_no in 0..vec.len() {
            let int = vec[thread_no];
            threads.push(thread::spawn(move || {
                let test = collect_pid_data(&int);
                match test{
                    Ok(()) =>println!(""),
                    Err(error) => panic!("{:?}", error),
                };   
            }))
        }
        for t in threads {
            t.join().unwrap();
        }
    }    
}
