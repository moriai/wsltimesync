use std::{error,env};
use std::fs::{self,File};
use std::time::{SystemTime,SystemTimeError};
use libc::{CLOCK_REALTIME,timespec};
use chrono::prelude::{DateTime,Local};

#[cfg(not(target_os = "macos"))]
use libc::clock_settime;

#[cfg(target_os = "macos")]
use libc::{clockid_t,c_int};
#[cfg(target_os = "macos")]
extern "C" {
    pub fn clock_settime(clk_id: clockid_t, tp: *const timespec) -> c_int;
}

trait SetTime {
    fn settime(&self) -> Result<(), SystemTimeError>;
}

impl SetTime for SystemTime {
    fn settime(&self) -> Result<(), SystemTimeError> {
        let ts = match self.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(unixtime) => 
                timespec {
                    tv_sec: unixtime.as_secs() as i64,
                    tv_nsec: unixtime.subsec_nanos() as i64
                },
            Err(e) => return Err(e)
        };
        let _ = unsafe { clock_settime(CLOCK_REALTIME, &ts) };
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
    let args: Vec<String> = env::args().collect();
    let timestamp = if args.len() < 2 { default_timestamp() } else { &args[1] };

    println!("timestamp file: {}", timestamp);

    let stime = SystemTime::now();
    { let _ = File::create(timestamp)?; }
    let etime = SystemTime::now();
    let estimated = stime + (etime.duration_since(stime)?/2);

    #[cfg(debug)]
    println!("    stime: {}", DateTime::<Local>::from(stime).to_rfc3339());
    #[cfg(debug)]
    println!("    etime: {}", DateTime::<Local>::from(etime).to_rfc3339());

    let metadata = fs::metadata(timestamp)?;
    if let Ok(mtime) = metadata.modified() {
        println!("timestamp: {}", DateTime::<Local>::from(mtime).to_rfc3339());
        println!("  current: {}", DateTime::<Local>::from(estimated).to_rfc3339());
        if let Ok(diff) = mtime.duration_since(estimated) {
            println!("difference: {:?} behind", diff);
            let newtime = SystemTime::now() + diff;
            newtime.settime()?;
        } else {
            let diff = estimated.duration_since(mtime)?;
            println!("differncee: {:?} ahead", diff);
        }
    } else {
        println!("Not supported on this platform");
    }

    Ok(())
}
