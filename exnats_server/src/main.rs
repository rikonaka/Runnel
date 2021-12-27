// use std::io::Read;
// use std::fs::File;
use std::process;

extern crate ini;
use ini::Ini;
use clap::{Arg, App};

mod liblistener;

struct GlobalConfig {
    listen_address: String,
    port: i32,
    token: String,
}

impl GlobalConfig {
    fn new() -> GlobalConfig {
        let global_config = GlobalConfig {
            listen_address: String::from("127.0.0.1"),
            port: 11080,
            token: String::from("opopop"),
        };
        return global_config;
    }
}

struct AllConfig {
    global_config: GlobalConfig,
}

impl AllConfig {
    fn new() -> AllConfig {
        let all_config = AllConfig {
            global_config: GlobalConfig::new(),
        };
        return all_config;
    }
}

fn parse_config(config_file: String) -> AllConfig {

    let conf = Ini::load_from_file(config_file).unwrap();
    let mut all_config: AllConfig = AllConfig::new();
    let global_config = &mut all_config.global_config;
    for (sec, prop) in &conf {
        // println!("Section: {:?}", sec);
        match sec {
            Some(s) => {
                // println!("{}", s);
                if s == "global" {
                    // some global config
                    for (key, value) in prop.iter() {
                        match key {
                            "listen_address" => global_config.listen_address = value.to_string(),
                            "port" => global_config.port = value.to_string().parse::<i32>().unwrap(),
                            "token" => global_config.token = value.to_string(),
                            _ => {
                                println!("Unknown config parameter: {}", value);
                                process::exit(1);
                            }
                        };
                    }
                }
            },
            _ => println!("Please provide correct config file!"),
        }
    }

    return all_config;
}

#[test]
fn test_parse_config() {
    // cargo test test_parse_config -- --nocapturelisten_address
    let all_config = parse_config("config.ini".to_string());
    println!("global config {:?}", all_config.global_config.listen_address);
    println!("global config {:?}", all_config.global_config.port);
    println!("global config {:?}", all_config.global_config.token);
}

fn main() {
    let matches = App::new("exnats_server")
        .version("0.1.0")
        .author("RikoNaka <xxy1836@gmail.com>")
        .about("Exposes local servers behind NATs and firewalls to the public internet over secure tunnels.")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        // println!("Value for config: {}", c);
        let all_config = parse_config(c.to_string());
        let global_config = all_config.global_config;
        // println!("{}", all_config.global_config.port);
        liblistener::server(global_config.listen_address, global_config.port, global_config.token);
    }
}