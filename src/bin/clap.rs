use clap::{App, Arg, SubCommand};

fn main() {
    let a = App::new("myApp")
        .version("1.0")
        .author("igonejack@gmail.com")
        .about("tool for testing")
        .args(&[
            Arg::from_usage("-c, --config=[FILE] 'set config file path'"),
            Arg::from_usage("-v, --verbose 'verbose printing'")
        ]);

    let m = a.get_matches();

    let feed = m.value_of("config").unwrap_or("feed.txt");
}
