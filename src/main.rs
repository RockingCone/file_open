use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;
//use std::fs;
use std::process;
//use std::error::Error;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut preferences = HashMap::new();

    preferences.insert(String::from("default"), String::from("nvim"));

    let target = Target::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let program = preferences.get(&target.file_extension).unwrap_or_else( ||
        preferences.get("default").unwrap_or_else( || {
            eprintln!("No default open program");
            process::exit(1);
        })
    );

    let _ = Command::new(program).arg(target.file_path + "." + &target.file_extension).exec();
} 

pub struct Target {
    pub file_extension: String,
    pub file_path: String,
}

impl Target {
    fn build(args: &[String]) -> Result<Target, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let full_file_string = args[1].clone();
        
        let mut file_extension = String::new();
        let mut file_path = String::new();
        
        let mut extension_found = false;

        for c in full_file_string.chars().rev() {
            if !extension_found && (c == '.') {
                extension_found = true;   
            } else if !extension_found {
                file_extension = c.to_string() + &file_extension;
            } else {
                file_path = c.to_string() + &file_path;
            }
        }

        Ok(Target { file_extension, file_path })
    }
}
