use libc::{clockid_t,c_int,timespec};

extern "C" {
    pub fn clock_settime(clk_id: clockid_t, tp: *const timespec) -> c_int;
}

pub mod time {
    use nix::time::ClockId;
    use nix::sys::time::TimeSpec;
    use nix::errno::Errno;

    #[inline]
    pub fn clock_settime(clock_id: ClockId, timespec: TimeSpec)
    -> Result<(), nix::Error> {
        let ret = unsafe { super::clock_settime(clock_id.as_raw(), timespec.as_ref()) };
        Errno::result(ret).map(drop)
    }
}
