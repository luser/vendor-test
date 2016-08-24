extern crate chrono;
extern crate chrono_humanize;
extern crate rand;

use chrono::{Local, Duration};
use chrono_humanize::HumanTime;

fn main() {
    println!("Hello, world: {}, {}", rand::random::<u8>(), HumanTime::from(Local::now() + Duration::days(35)));
}
