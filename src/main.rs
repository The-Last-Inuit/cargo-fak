#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

static ADR: &str = "#### Context
#### Decision
#### Status
#### Consequences
";

fn main() -> std::io::Result<()> {
    let key: u64;
    let title: &str;

    let matches = App::new("cargo-fak")
        .bin_name("cargo")
        .setting(AppSettings::SubcommandRequired)
        .version(concat!("v", crate_version!()))
        .author("louise <email@luisignac.io>")
        .subcommand(SubCommand::with_name("adr")
            .about("Cargo Faaaaaak! is an ADR tool for your Rust projects. You don't need help, you are human, you are fine. What you need is better tools, Cargo Faaaaaak! aims to help you out.")
            .arg(Arg::with_name("TITLE")
                .help("A title of your ADR")
                .required(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("adr") {
        if matches.is_present("TITLE") {
            title = matches.value_of("TITLE").unwrap();
        } else {
            title = &"No title"
        }
    } else {
        title = &"No title"
    }

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => key = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    let directory: &str = "./docs/adr";
    fs::create_dir_all(directory)?;
    let filepath = format!("{}/{}_{}.md", directory, key, title).to_string();
    let path = Path::new(&filepath);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    file.write_all(ADR.as_bytes())
}
