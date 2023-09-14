use std::thread;
use std::time::Duration;

pub fn delay_for_miliseconds(ms : u64){
    let duration = Duration::from_millis(ms);
    thread::sleep(duration);
}