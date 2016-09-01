#[macro_use]
extern crate bitflags;
extern crate clap;

use clap::App;

bitflags! {
    flags Flags: u32 {
        const FLAG_A       = 0b00000001,
        const FLAG_B       = 0b00000010,
    }
}

fn main() {
    let e1 = FLAG_A | FLAG_B;
    let app = App::new("vendor-test")
        .version(env!("CARGO_PKG_VERSION"))
        .args_from_usage("-d --do-something 'do something'");
    let matches = app.get_matches();
    if matches.is_present("do-something") {
        println!("do something! {:?}", e1);
    } else {
        println!("not doing anything! {:?}", e1);
    }
}
