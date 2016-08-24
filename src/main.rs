extern crate rand;
extern crate time;

fn main() {
    println!("Hello, world: {}, {}", rand::random::<u8>(), time::now().ctime());
}
