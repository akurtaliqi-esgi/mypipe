extern crate clap;

use clap::{Arg, App};
use std::process::Command;
use std::io::{self, Write};

fn main() {
    let matches = App::new("mypipe")
                          .version("1.0")
                          .author("Ajtene K. <ajtene.kurtaliqi@gmail.com>")
                          .about("Does awesome things")
                          .arg(
                                Arg::with_name("input")
                                .takes_value(true)
                                .long("in")
                                .required(true)
                                )
                           .arg(
                                Arg::with_name("output")
                                .takes_value(true)
                                .long("out")
                                .required(true)
                                )                           
                          .get_matches();

    let input_command_string = matches.value_of("input").unwrap();
    let output_command_string = matches.value_of("output").unwrap();

    let input_command_result = Command::new(String::from(input_command_string))
                        .output()
                        .expect("you failed");
                        
    io::stdout().write_all(&(input_command_result).stdout).unwrap();

    println!();
    
    let output_command_reqult = Command::new(output_command_string)
                        .output()
                        .expect("you failed");


    io::stdout().write_all(&(output_command_reqult).stdout).unwrap();
}