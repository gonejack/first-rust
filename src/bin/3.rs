use std::io::{stdout, BufWriter};
use first_rust::main;

fn main() {
    main::abc();
    main::def();

    let stdout = stdout();
    let msg = String::from("xx");
    let width = msg.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    let result = ferris_says::say(msg.as_bytes(), width, &mut writer);

    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        dbg!(result.unwrap_err());
    }
}
