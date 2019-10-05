// Rust SHell
// v.0.1.0
// By Jacob Haig (jhaig343@gmail.com)

use std::path::Path;
use std::env;
use std::process::Command;
use std::io::{self, Write, BufRead};
mod utility;


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
			let dir_change = env::set_current_dir(&root);
			if dir_change.is_err() {
				let error = dir_change.unwrap_err();
				println!("cd: {}: {}.", root.to_string_lossy(), error);
			}
			continue;
		}

		if execute == "vim" || execute == "nano" { //Need to spawn a child process, not wait to collect exit output
			let mut program = Command::new(execute).args(args).spawn().expect("Failed to execute command");
			program.wait().expect("child process terminated abnormally");
			continue;
		}

		let output = Command::new(execute).args(args).output();

		// Error message syntax: [COMMAND]: [Errormsg]
		if output.is_err() {
			let failed_output = output.unwrap_err();
			println!("\x1b[31m{}: {}\x1b[0m", execute, failed_output );
			continue;
		}
		else {
			let success_output = output.expect("Shell failed to execute command.");
			if execute == "ls" {
				utility::test_ls_pretty_print(&success_output);
				continue;
			}
			
			utility::pretty_print(success_output);
		}
	}
	
}