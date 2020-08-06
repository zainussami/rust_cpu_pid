use psutil::process::Process;
use std::{thread, time};

fn delay(millis: u64) {
    let timeout = time::Duration::from_millis(millis);
    thread::sleep(timeout);
}

fn main() {

    let mut pid_proc = Process::new(46717).expect("Failed accessing process");
    let percent_cpu = pid_proc.cpu_percent().unwrap();
    println!("CPU% {:.02}", percent_cpu);
    loop{
        delay(1000);
        let percent_cpu = pid_proc.cpu_percent().unwrap();
        println!("CPU% {:.02}", percent_cpu);
    }

}