extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
    let matches = App::new("mypipe")
                          .version("1.0")
                          .author("Ajtene K. <ajtene.kurtaliqi@gmail.com>")
                          .about("Does awesome things")
                          .arg(
                                Arg::with_name("input")
                                .takes_value(true)
                                .long("--in")
                                .requires("out")
                                )
                           .arg(
                                Arg::with_name("output")
                                .takes_value(true)
                                .long("--out")
                                .requires("output")
                                )                           
                          .get_matches();

    let cmd_fortune = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let cmd_fortune = 
                Command::new(cmd_fortune)
                    .output()
                    .expect("you failed");

    let msg = String::from_utf8_lossy(&cmd_fortune.stdout).to_string();

    let cmd_cowsay =   
                Command::new(output)
                    .arg(msg)
                    .output()
                    .expect("you failed");

    println!("{}", String::from_utf8_lossy(&cmd_cowsay.stdout).to_string());

}
