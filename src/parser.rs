use clap::{Arg, App};
//Command line parser using Clap
pub fn console_parser()-> (bool,String) {
    let matches = App::new("PID CPU Utilization")
    .version("1.0.0")
    .author("Zain Ansari <zainussami@gmail.com>")
    .about("Monitors CPU Utilization for specified PIDs")
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