use std::{error,env};
use std::fs::{self,File};
use std::time::SystemTime;
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

fn main() -> Result<(), Box<dyn error::Error>> {    
    let timestamp = env::var("USERPROFILE")? + "/.wsltimestamp";

    let start_time = SystemTime::now();
    { let _ = File::create(&timestamp)?; }
    let end_time = SystemTime::now();
    let estimated = start_time + (end_time.duration_since(start_time)?/2);

    let metadata = fs::metadata(&timestamp)?;
    if let Ok(mtime) = metadata.modified() {
        if let Ok(diff) = mtime.duration_since(estimated) {
            println!("difference: {:?} behind", diff);
            let new_time = SystemTime::now() + diff;
            new_time.set()?;
        } else {
            let diff = estimated.duration_since(mtime)?;
            println!("difference: {:?} ahead", diff);
        }
    } else {
        println!("Not supported on this platform");
    }

    Ok(())
}
