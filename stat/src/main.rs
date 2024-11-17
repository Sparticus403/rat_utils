fn handle_opts(opts: &[&str]) {
    for o in opts {
        match *o {
            "--help" => println!("help text"),
            _ => (),
        };
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("provide file to stat");
        std::process::exit(1);
    }

    let mut options: Vec<&str> = Vec::new();
    for arg in &args {
        if let Some(t) = arg.find('-') {
            if t == 0 {
                options.push(arg.as_str());
            }
        }
    }

    println!("provided options: {:?}", options);
    handle_opts(&options);
}
