use psutil::process::Process;
use std::{thread, time};
use std::time::Duration;
use chrono::prelude::*;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use clap::{Arg, App};

const SAMPLES:u32 = 60;    // Number of Samples 
const INTERVAL:u64 = 1000;    // Sampling Interval in Miliseconds    


fn delay(millis: u64) {
    let timeout = time::Duration::from_millis(millis);
    thread::sleep(timeout);
}

fn display_welcome() {
        //Display Message 
        println!("RUST Process CPU utilization");
        println!("Please Enter a Process ID (Leave Blank to Monitor this application): ");

}

//Command line parser using Clap
fn console_parser()-> (bool,String) {
    let matches = App::new("PID CPU Utilization")
    .version("1.0.0")
    .author("Zain Ansari <zainussami@gmail.com>")
    .about("Monitors CPU Utilization")
    .arg(Arg::with_name("batch")
             .short("b")
             .long("batch")
             .takes_value(true)
             .help("List of PIDs seperated by commas e.g. -b 646,323,55,665"))
    .arg(Arg::with_name("interactive")
             .short("i")
             .long("interactive")
             .takes_value(false)
             .help("Launches Interactive Mode"))
    .get_matches();
    let mut interactive = false;
    if matches.is_present("interactive") {
        interactive = true;    
    }    
    let mut batch = "";
    if matches.is_present("batch") {
        batch = matches.value_of("batch").unwrap();
    }    
    (interactive,batch.to_string())
}
fn interactive_mode(){

}

//Funtion Handle Errors in PID inputs (Valid Input is u32) 
fn parse_pids(reader: String) -> Vec<u32>{
    let split_string = reader.split(',');
    let (numbers, errors): (Vec<_>, Vec<_>) = split_string
        .into_iter()
        .map(|s| s.parse::<u32>())
        .partition(Result::is_ok);
        let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
        println!("Errors: {:?}", errors);
    numbers
}

fn print_mypid (pid : &u32){
    //Error Handling Check if the process Exists
    let mut pid_proc = Process::new(*pid);
    let pid_proc = match pid_proc {
        Ok(pid_acquired) => {
            println!("Collecting CPU utilization Data for {:.100}", pid_acquired.name().unwrap());

        },
        //Thread Fails But does not Panic and Reports an error
        Err(error) => println!("{:?}", error),
    };

}

fn main() {

    let (mode,pid_lists)= console_parser();
    //Check the Mode
    if mode == true{
        print!("{}[2J", 27 as char);
        loop{
            display_welcome();
            interactive_mode();
        }
    }
    else{
        //Convert PID List from Ints
        //Spawn Multiple Threads
        let vec = parse_pids(pid_lists);        
        println!("Valid PIDs {:?} :", vec);
        let mut threads = Vec::new();
        for thread_no in 0..vec.len() {
            let int = vec[thread_no];
            threads.push(thread::spawn(move || {
                print_mypid(&int);
            }))
        }
        for t in threads {
            t.join().unwrap();
        }
    }    
}
