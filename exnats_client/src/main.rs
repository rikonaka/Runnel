use std::process;

extern crate ini;
use ini::Ini;
use clap::{Arg, App};

mod libclient;

struct GlobalConfig {
    server: String,
    port: i32,
    token: String,
}

impl GlobalConfig {
    fn new() -> GlobalConfig {
        let global_config = GlobalConfig {
            server: String::from("127.0.0.1"),
            port: 11080,
            token: String::from("opopop"),
        };
        return global_config;
    }
}

struct IndivConfig {
    name: String,
    expose_port: i32,
    local_port: i32,
}

impl IndivConfig {
    fn new() -> IndivConfig {
        let indiv_config = IndivConfig{
            name: String::from("init_value"),
            expose_port: 11081,
            local_port: 8080,
        };
        return indiv_config;
    }
}

struct AllConfig {
    global_config: GlobalConfig,
    indiv_config_vec: Vec<IndivConfig>,
}

impl AllConfig {
    fn new() -> AllConfig {
        let all_config = AllConfig {
            global_config: GlobalConfig::new(),
            indiv_config_vec: Vec::new(),
        };
        return all_config;
    }
}

fn parse_config(config_file: String) -> AllConfig {

    let conf = Ini::load_from_file(config_file).unwrap();
    let mut all_config: AllConfig = AllConfig::new();
    let global_config = &mut all_config.global_config;
    let indiv_config_vec = &mut all_config.indiv_config_vec;
    for (sec, prop) in &conf {
        // println!("Section: {:?}", sec);
        match sec {
            Some(s) => {
                // println!("{}", s);
                if s == "global" {
                    // some global config
                    for (key, value) in prop.iter() {
                        match key {
                            "server" => global_config.server = value.to_string(),
                            "port" => global_config.port = value.to_string().parse::<i32>().unwrap(),
                            "token" => global_config.token = value.to_string(),
                            _ => {
                                println!("Unknown config parameter: {}", value);
                                process::exit(1);
                            }
                        };
                    }
                }
                else {
                    // normal config here
                    let mut indiv_config = IndivConfig::new();
                    indiv_config.name = s.to_string();
                    for (key, value) in prop.iter() {
                        // println!("{:?}:{:?}", key, value);
                        match key {
                            "expose_port" => indiv_config.expose_port = value.to_string().parse::<i32>().unwrap(),
                            "local_port" => indiv_config.local_port = value.to_string().parse::<i32>().unwrap(),
                            _ => {
                                println!("Unknown config parameter: {}", value);
                                process::exit(1);
                            },
                        };
                    }
                    indiv_config_vec.push(indiv_config);
                }
            },
            _ => println!("Please provide correct config file!"),
        }
    }

    return all_config;
}

#[test]
fn test_parse_config() {
    // cargo test test_parse_config -- --nocapture
    let all_config = parse_config("config.ini".to_string());
    println!("global config {:?}", all_config.global_config.server);
    println!("global config {:?}", all_config.global_config.port);
    println!("global config {:?}", all_config.global_config.token);
    let indiv_config_vec = all_config.indiv_config_vec;
    for ind in indiv_config_vec {
        println!("indiv config {:?}", ind.name);
        println!("indiv config {:?}", ind.expose_port);
        println!("indiv config {:?}", ind.local_port);
    }
}

fn main() {
    let matches = App::new("exnats_client")
        .version("0.1.0")
        .author("RikoNaka <xxy1836@gmail.com>")
        .about("Exposes local servers behind NATs and firewalls to the public internet over secure tunnels.")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .takes_value(true))
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        // println!("Value for config: {}", c);
        let all_config = parse_config(c.to_string());
        // println!("{}", all_config.global_config.port);
        libclient::client(&all_config.global_config.server, &all_config.global_config.port);
    }
}