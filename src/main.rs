use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;

fn main() {
    // Collect user arguments
    let args: Vec<String> = env::args().collect();
    
    // Get user home directory to use configs
    let home = match env::home_dir() {
        Some(dir) => dir,
        None => {
            eprintln!("Gobbo");
            process::exit(1)
        },
    };
    let home = home.display();
    
    // Copy config file into hashmap. Create default file is none exists.
    let config_file = format!("{home}/.config/open/config.toml");
    if !fs::metadata(&config_file).is_ok() {
        let mut default_config = String::from("nano = ");
        default_config.push('"');
        default_config.push_str("default");
        default_config.push('"');

        let config_dir = format!("{home}/.config/open");
        fs::create_dir(config_dir).unwrap_or_else( |err| {
            eprintln!("Error creating config directory: {err}");
            process::exit(1);
        });
        fs::write(&config_file, default_config).unwrap_or_else( |err| {
            eprintln!("Error creating config file: {err}");
            process::exit(1);
        });
    }
    let config_file = fs::read_to_string(config_file).unwrap_or_else( |err| {
        eprintln!("Error accessing config file: {err}");
        process::exit(1);
    });
    let config = parse_config(&config_file);

    // Parse user arguments and determine required program to open file
    let target = Target::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });
    let program = config.get(&target.file_extension).unwrap_or_else( ||
        config.get("default").unwrap_or_else( || {
            eprintln!("No default open program");
            process::exit(1);
        })
    );
    
    // Open user specified file using the program drawn from config
    let _ = Command::new(program)
        .arg(target.file_path + "." + &target.file_extension)
        .exec();
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

        //let full_file_string = args[1].clone();
        
        let mut file_path = String::new();
        let mut file_extension = String::new();

        let file_split: Vec<&str> = args[1].split('.').collect();
        file_path.push_str(&file_split[0]);
        for str in file_split {
            if str != &file_path {
                file_extension.push_str(str);
            }
        }

        Ok(Target { file_extension, file_path })
    }
}

pub fn parse_config(config_file: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    
    for line in config_file.lines() {
        let args: Vec<&str> = line.trim().split('=').collect();
        let program = args[0].trim();

        let extensions: Vec<&str> = args[1].trim().split(&['[', ' ', ',', ']'][..]).collect();
        for extension in extensions {
            config.insert(extension.trim().trim_matches('"').to_string(), program.to_string());
        }
    }
    

    config
}
