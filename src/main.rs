extern crate chrono;
extern crate chrono_humanize;
extern crate rand;

use chrono::in_35_days;
use chrono_humanize::HumanTime;

fn main() {
    println!("Hello, world: {}, {}", rand::random::<u8>(), HumanTime::from(in_35_days()));
}
