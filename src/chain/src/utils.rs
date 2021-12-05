use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn time_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
