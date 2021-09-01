use clap::clap_app;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "igonejack@gmail.com")
        (about: "html to email")
        (@arg CONFIG: -c --config "Sets a custom config file")
    ).get_matches();

    println!("{:#?}", matches);
}
