extern crate clap;
use clap::{Arg, App, SubCommand};

mod valifunc;

//use valifunc::{*};

const DEFAULT_VERSION:&str = "0.1.0";

const SUBCMD_PING:&str = "ping";
    const PING_HOST:&str = "ping_host";
    const PING_DPORT:&str = "ping_dport";
    const PING_PROTO_TYPE:&str = "ping_proto_type";
    const PING_PROXY:&str = "ping_proxy";





fn main() {
    let matches = App::new("rsping")
                            .version(DEFAULT_VERSION)
                            .author("wuqian <wuqianking123@gmail.com>")
                            .about("a super ping implied by rust")
                            .arg(Arg::with_name("v")
                                .short("v")
                                .multiple(true)
                                .help("Sets the level of verbosity"))
                            .subcommand(SubCommand::with_name(SUBCMD_PING)
                                .about("basic ping func supporting icmp, udp, tcp, http, and proxy ")
                                .version(DEFAULT_VERSION)
                                .author("wuqian <wuqianking123@gmail.com>")
                                .arg(Arg::with_name(PING_PROTO_TYPE)
                                    .required(true)
                                    .index(1)
                                    .help("specify protcol type: tcp, udp, http"))
                                .arg(Arg::with_name(PING_DPORT)
                                    .short("p")
                                    .long("dport")
                                    .validator(valifunc::vali_port)
                                    .required_ifs(&[(PING_PROTO_TYPE as &str, "tcp"), (PING_PROTO_TYPE as &str, "udp"), (PING_PROTO_TYPE as &str, "http")])
                                    .help("specify destination port"))
                                .arg(Arg::with_name(PING_PROXY)
                                    .long("proxy")
                                    //.validator(f)
                                    .help("specify destination port"))
                                .arg(Arg::with_name(PING_HOST)
                                    .short("d")
                                    .long("dip")
                                    .validator(valifunc::vali_ip)
                                    .required(true)
                                    .help("destination ip")))
                            .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    
    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    // let ii:Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    // for i in ii.iter(){
    //     println!("aaaaaaaaa{}", i);
    // }
    
    //println!("Using input file: {}", matches.values_of("INPUT").unwrap().collect());
    //println!("Using output file: {}", matches.value_of("OUTPUT").unwrap());


    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    // if let Some(matches) = matches.subcommand_matches("test") {
    //     if matches.is_present("debug") {
    //         println!("Printing debug info...");
    //     } else {
    //         println!("Printing normally...");
    //     }
    // }

    // more program logic goes here...
}