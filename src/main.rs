// Rust SHell
// v.0.2.0
// By Jacob Haig (jhaig343@gmail.com)

use std::path::Path;
use std::env;
use std::process::Command;
use std::io::{self, Write, BufRead};
mod utility;


fn main() {


	loop {
		let printDir = env::current_dir();
		assert!(printDir.is_ok());

		print!("\x1b[34m{}\x1b[0m=>$", printDir.ok().unwrap().to_string_lossy());

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
			if !(env::set_current_dir(&root).is_ok()) {
				let cd_err = env::set_current_dir(&root);
				
				utility::handle_err(cd_err, execute);
			}
			continue;
		}

		let output = Command::new(execute).args(args).spawn();

		// Error message syntax: [COMMAND]: [Errormsg]
		// \x1b[Xm , where x is the ANSI color code colors following text output - 31 is red
		// 0 clears color code
		if output.is_err() {
			let failed_output = output.unwrap_err();
			println!("\x1b[31m{}: {}\x1b[0m", execute, failed_output );
			continue;
		}
		else {
			let success_output = output.expect("Shell failed to execute command.");
			// if execute == "ls" {
			// 	utility::test_ls_pretty_print(success_output);
			// 	continue;
			// }
			
			utility::pretty_print(success_output);
		}
	}
	
}