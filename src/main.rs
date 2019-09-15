// Rust SHell
// v.0.1.0
// By Jacob Haig (jhaig343@gmail.com)

use std::path::Path;
use std::env;
use std::process::Command;
use std::io::{self, Write, BufRead};



fn main() {


	loop {
		print!("=>$");
		io::stdout().flush().unwrap();

		let buffer = io::stdin();
		
		let mut line = String::new();

		buffer.lock().read_line(&mut line).unwrap();
		// pop() will remove the last character in the string, 
		// which in this case is \n character
		line.pop();

		// separate string into words (split on spaces)
		let split = line.split(" ");

		let mut args = split.collect::<Vec<&str>>();
		let execute = args.remove(0);

		if line == "exit" {
			break;
		}
		if execute == "cd" { //cd is a shell builtin, not a /bin program
			let root = Path::new(args[0]);
			assert!(env::set_current_dir(&root).is_ok());
			continue;
		}

		let output = Command::new(execute).args(args).env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15").output();

		// Error message syntax: [COMMAND]: [Errormsg]
		if output.is_err() {
			let failed_output = output.unwrap_err();
			println!("{}: {}", execute, failed_output );
			continue;
		}
		else {
			let success_output = output.expect("Shell failed to execute command.");
		
			if !success_output.status.success() {
				let err = success_output.stderr;
				print!("{}", String::from_utf8(err).ok().unwrap() );
			}
			else {
				let result = success_output.stdout;
				print!("{}", String::from_utf8(result).ok().unwrap() );
			}
		}
		
		
	}
	
}