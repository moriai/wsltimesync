use std::time::{SystemTime,SystemTimeError};
use libc::{CLOCK_REALTIME,timespec,clockid_t,c_int};

extern "C" {
    pub fn clock_settime(clk_id: clockid_t, tp: *const timespec) -> c_int;
}

pub trait SetTime {
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
