use std::error::Error;

use clap::{App, Arg, ArgMatches};
use log::{info, error, LevelFilter};

use html_to_email::cmd::HtmlToEmail;

fn args() -> ArgMatches<'static> {
    App::new("myApp")
        .version("1.0")
        .author("igonejack@gmail.com")
        .about("tool for testing")
        .args(&[
            Arg::from_usage("-c, --config=[FILE] 'set config file path'").default_value(""),
            Arg::from_usage("-v, --verbose 'verbose printing'"),
            Arg::with_name("input").multiple(true)
        ])
        .get_matches()
}

fn conv(html: String) -> Result<(), Box<dyn Error>> {
    HtmlToEmail::new(html).run()
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    let args = args();
    let htms = args.values_of("input").unwrap_or(Default::default());

    if htms.to_owned().count() == 0 {
        error!("not html given");
        return;
    }

    for htm in htms {
        info!("process {}", htm);

        let res = conv(htm.to_string());
        if res.is_err() {
            error!("parse {} failed: {}", htm, res.err().unwrap());
        }
    }
}

