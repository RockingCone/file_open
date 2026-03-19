use std::process::Command;
use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let target = Target::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{target:?}"); 

    //Command::new("nvim").arg(&args[1]).status().expect("failed to execute process");
}

#[derive(Debug)]
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
