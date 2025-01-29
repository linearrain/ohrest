pub mod protocols;
pub mod layers;
pub mod filtering;
pub mod io;

use crate::io::interpret_parameters;
use crate::protocols::find_packets;


// PARAMETERS ENUM TO BE USED IN TWO PARTS:
// 1. IN THE IO FUNCTION TO PARSE THE ARGUMENTS
// 2. IN THE FILTERING FUNCTION TO FILTER THE PACKETS

#[derive(Debug)]
pub enum Parameters { 
    IpAddress(Vec<String>),
    Port(Vec<u16>),
    Interface(Vec<String>),
    Protocol(Vec<protocols::Protocol>),
    NoParameter,
}



// FUNCTION FOR GETTING THE COLOR TO
// BE USED IN THE PRINTING FUNCTIONS
// OVER THE WHOLE PROGRAM

fn get_color<'a>(color_code : u8) -> &'a str {
    match color_code {
        1 => "\x1b[1m",         // BOLD
        2 => "\x1b[38;5;154m",  // Green-Yellow
        3 => "\x1b[38;5;198m",  // Red
        4 => "\x1b[38;5;147m",  // Blue
        5 => "\x1b[38;5;226m",  // Yellow
        6 => "\x1b[38;5;214m",  // Orange
        7 => "\x1b[38;5;213m",  // Purple
        _ => "\x1b[0m",         // RESET
    }
}

fn print_program_name() {
    print!("\x1b[38;5;153m[OHREST / ОРЕСТ]\x1b[0m ");
}

fn print_error() {
    print!("\x1b[38;5;197m[OHREST PANICS]\x1b[0m ");
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let parameters = interpret_parameters(&args);

    find_packets(parameters);
}
