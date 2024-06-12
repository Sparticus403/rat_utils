use std::env::args;

fn print_help() {
    println!("Usage: yes [STRING]...");
    println!("  or:  yes OPTION");
    println!("Repeatedly output a line with all specified STRING(s), or 'y'.\n");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit (unimplemented)\n");
    println!("RAT UTILS project avaiable: <https://www.github.com/Sparticus403/rat_utils>");
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        if args[1] != "--help" || args[1] == "--version" {
            loop {
                for i in 1..args.len() {
                    print!("{} ", &args[i]);
                }
                print!("\n");
            }
        } else {
            match args[1].as_str() {
                "--help" => print_help(),
                "--version" => unimplemented!(),
                _ => (),
            };
        }
        std::process::exit(0);
    }
    loop {
        println!("y");
    }
}
