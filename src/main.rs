#[macro_use]
extern crate clap;

use std::fs;
use std::path::Path;
use clap::{App, AppSettings, SubCommand, Arg};
use std::time::SystemTime;
use std::io::Write;

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

    println!("{:?} {:?}", key, title);

    let directory: &str = "./docs/adr";
    fs::create_dir_all(directory)?;
    let filepath = Path::new(&format!("{}/{}_{}.md", directory, key, title).to_string());
    let mut file = std::fs::File::create(filepath).expect("create failed");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");

    Ok(())
}
