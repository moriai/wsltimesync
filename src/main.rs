use std::{error,env};
use std::fs::{self,File};
use std::time::SystemTime;
use nix::time::ClockId;
use nix::sys::time::TimeSpec;
#[cfg(not(target_os = "macos"))]
use nix::time;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::time;

trait SetTime {
    fn set_time(&self) -> Result<(), Box<dyn error::Error>>;
}

impl SetTime for SystemTime {
    fn set_time(&self) -> Result<(), Box<dyn error::Error>> {
        let unixtime = self.duration_since(SystemTime::UNIX_EPOCH)?;
        time::clock_settime(ClockId::CLOCK_REALTIME, TimeSpec::from(unixtime))?;
        Ok(())
    }
}

fn default_timestamp() -> &'static str {
    if let Ok(dir) = env::var("USERPROFILE") {
        let fname = dir + "/.wsltimestamp";
        Box::leak(fname.into_boxed_str())
    } else {
        "timestamp"
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {    
    let timestamp = default_timestamp();

    let stime = SystemTime::now();
    { let _ = File::create(timestamp)?; }
    let etime = SystemTime::now();
    let estimated = stime + (etime.duration_since(stime)?/2);

    let metadata = fs::metadata(timestamp)?;
    if let Ok(mtime) = metadata.modified() {
        if let Ok(diff) = mtime.duration_since(estimated) {
            println!("difference: {:?} behind", diff);
            let newtime = SystemTime::now() + diff;
            newtime.set_time()?;
        } else {
            let diff = estimated.duration_since(mtime)?;
            println!("differncee: {:?} ahead", diff);
        }
    } else {
        println!("Not supported on this platform");
    }

    Ok(())
}
