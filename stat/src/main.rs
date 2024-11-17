use std::{fs, os::linux::fs::MetadataExt};

fn handle_opts(opts: &[&str]) {
    for o in opts {
        match *o {
            "--help" => print_help(),
            _ => (),
        };
    }
}

fn print_help() {
    println!("Usage: stat [OPTION]... FILE...");
    println!("Display file or file system status.\n");
    println!("Mandatory arguments to long options are mandatory for short options too.");
    println!("      --help        display this help and exit");

    std::process::exit(0);
}

fn stat(filename: &str) {
    println!("  File: {filename}");
    let metadata = fs::metadata(filename).unwrap();
    println!("  Size: {}\t\tBlocks: {}\tIO Block: {}\t{}", metadata.len(),metadata.st_blocks(), metadata.st_blksize(), if metadata.is_dir(){"directory"} else {"regular file"});

    let dev_1 = metadata.st_dev() >> 8;
    let dev_2 = metadata.st_dev() ^ (dev_1 << 8);

    println!("Device: {dev_1},{dev_2}\tInode: {}\t\tLinks: {}", metadata.st_ino(), metadata.st_nlink());

    println!("Access: {}", metadata.st_atime_nsec());
    println!("Modify: {}", metadata.st_mtime_nsec());
    println!("Change: {}", metadata.st_ctime_nsec());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("provide file to stat");
        std::process::exit(1);
    }

    let mut options: Vec<&str> = Vec::new();
    let mut files: Vec<&str> = Vec::new();
    for arg in &args {
        if let Some(t) = arg.find('-') {
            if t == 0 {
                options.push(arg.as_str());
            }
        } else { files.push(arg.as_str());}
    }

    // println!("provided options: {:?}", options);
    if options.len() != 0 {handle_opts(&options);}
    // println!("provided args: {:?}", args);
    for f in files {
        stat(f);
    }
}
