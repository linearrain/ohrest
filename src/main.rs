pub mod protocols;
pub mod layers;

fn get_color<'a>(color_code : u8) -> &'a str {
    match color_code {
        1 => "\x1b[1m",    // BOLD
        2 => "\x1b[38;5;154m",  // Green-Yellow
        3 => "\x1b[38;5;198m",  // Red
        4 => "\x1b[38;5;147m",  // Blue
        5 => "\x1b[38;5;226m",  // Yellow
        6 => "\x1b[38;5;214m",  // Orange
        7 => "\x1b[38;5;213m",  // Purple
        _ => "\x1b[0m",    // RESET
    }
}

fn print_program_name() {
    print!("\x1b[38;5;153m[OHREST / ОРЕСТ]\x1b[0m ");
}

fn print_error() {
    print!("\x1b[38;5;197m[OHREST PANICS]\x1b[0m ");
}

fn main() {

}

// TO OPTIMIZE THE PROGRAM THE TWO POSSIBLE SCENARIOS WILL BE ANALYZED:
    // 1. THE PROGRAM IS RUNNING WITHOUT ANY ARGUMENTS:
        // IT MEANS THAT WE WON'T HAVE ANY CONSTRAINTS
        // TO DISPLAY PROPERLY SUCH A PACKET AND DESCRIBE IT WE HAVE TO USE
        // DOWN-TOP APPROACH, MEANING THAT WE HAVE TO CHECK THE ETHERNET FRAME
        // FIRST, THEN THE IPV4, THEN OTHER PROTOCOLS
    // 2. THE PROGRAM IS RUNNING WITH ARGUMENTS:
        // IT MEANS THAT WE HAVE TO FILTER THE PACKETS ACCORDING TO THE ARGUMENTS
        // WE HAVE TO CHECK IF THE INCOMING PACKET MATCHES THE CRITERIA BY
        // COMPARING THE PACKET WITH TOP-LEVEL PROTOCOLS DIRECTLY
        // EXMAPLE: IF TCP PACKET GOES ON, IT CHECKS ONLY IF ETHERNET, IPv4 AND TCP
        // ARE VALID, NOT CHECKING THE OTHER TOP-LEVEL PROTOCOLS, AS IN THE FIRST CASE
