use std::{error,env};
use std::fs::{self,File};
use std::time::SystemTime;
use chrono::prelude::{DateTime,Local};
use nix::time::ClockId;
use nix::sys::time::TimeSpec;
#[cfg(target_os = "macos")] mod macos;
#[cfg(target_os = "macos")] use macos::time;
#[cfg(not(target_os = "macos"))] use nix::time;

trait SetTime {
    fn set(&self) -> Result<(), Box<dyn error::Error>>;
}

impl SetTime for SystemTime {
    fn set(&self) -> Result<(), Box<dyn error::Error>> {
        let unix_time = self.duration_since(SystemTime::UNIX_EPOCH)?;
        time::clock_settime(ClockId::CLOCK_REALTIME, TimeSpec::from(unix_time))?;
        Ok(())
    }
}

fn default_timestamp() -> String {
    if let Ok(dir) = env::var("USERPROFILE") {
        dir + "/.wsltimestamp"
    } else {
        String::from("timestamp")
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {    
    let args: Vec<String> = env::args().collect();
    let timestamp = if args.len() < 2 { default_timestamp() } else { String::from(&args[1]) };

    println!("timestamp file: {}", timestamp);

    let start_time = SystemTime::now();
    { let _ = File::create(&timestamp)?; }
    let end_time = SystemTime::now();
    let estimated = start_time + (end_time.duration_since(start_time)?/2);

    #[cfg(debug)]
    println!("     start: {}", DateTime::<Local>::from(start_time).to_rfc3339());
    #[cfg(debug)]
    println!("       end: {}", DateTime::<Local>::from(end_time).to_rfc3339());

    let metadata = fs::metadata(&timestamp)?;
    if let Ok(mtime) = metadata.modified() {
        println!(" timestamp: {}", DateTime::<Local>::from(mtime).to_rfc3339());
        println!("   current: {}", DateTime::<Local>::from(estimated).to_rfc3339());
        if let Ok(diff) = mtime.duration_since(estimated) {
            print!("difference: {:?} behind", diff);
            let new_time = SystemTime::now() + diff;
            if new_time.set().is_ok() {
                println!(" -- adjusted");
            } else {
                println!();
            }
        } else {
            let diff = estimated.duration_since(mtime)?;
            println!("difference: {:?} ahead", diff);
        }
    } else {
        println!("Not supported on this platform");
    }

    Ok(())
}
