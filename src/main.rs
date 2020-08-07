use psutil::process::Process;
use std::{thread, time};
use chrono::prelude::*;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn delay(millis: u64) {
    let timeout = time::Duration::from_millis(millis);
    thread::sleep(timeout);
}

fn main() {
    const SAMPLES:u32 = 60;    // Number od Samples 
    const INTERVAL:u64 = 1000;    // Sampling Interval in Miliseconds
    //Clear the Screen
    print!("{}[2J", 27 as char);
    loop{
        //Display Message 
        println!("RUST Process CPU utilization");
        println!("Please Enter a Process ID (Leave Blank to Monitor this application): ");

        //Get User Input For PID
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        if input_text.len()<2 {
            println!("My pid is {}", process::id());
            input_text = process::id().to_string();
        }
        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            //Check if the Input is Valid UnSigned 32 
            Ok(pid) => {println!("PID input: {}", pid);
            //If the Process Doesn't Exist the program panics
            let mut pid_proc = Process::new(pid).expect("Failed accessing process");
            println!("Collecting CPU utilization Data for {:.100}",pid_proc.name().unwrap());
            
            //Create a File to Write Output to
            let local: DateTime<Local> = Local::now();      
            let path_name = "data/".to_owned()  + &pid.to_string() +"_"+ 
                            &local.format("%Y-%m-%dT%H:%M:%S").to_string() + &".txt".to_owned() ;
            let path = Path::new(&path_name);
            let display = path.display();
                // Open a file in write-only mode, returns `io::Result<File>`
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };
            //Create File Header with process Name
            if let Err(e) = writeln!(file, "CPU utilization Data for {:.100}",pid_proc.name().unwrap()) {
                println!("Couldn't write to file: {}", e);
            }
            println!("Wrting to file {}", display);

            let mut counter = 0;
            while counter < SAMPLES { //Collect Data for 60 Seconds            
                delay(INTERVAL);
                //Get CPU Utilization
                let percent_cpu = pid_proc.cpu_percent().unwrap();  
                // Get Local Date Time     
                let local: DateTime<Local> = Local::now();
                print!(". ");
                io::stdout().flush().unwrap();      
                if let Err(e) = writeln!(file, "CPU%:{:>2.2}  TimeStamp:{:?}",percent_cpu,
                local.format("%Y-%m-%dT%H:%M:%S").to_string()) {
                    println!("Couldn't write to file: {}", e);
                }
                counter +=1;
            }
            println!("Succesfully wrote to file: {}", display);
        },
            Err(..) => println!("This was not a Valid PID input: {}", trimmed),
        };
    }
}
