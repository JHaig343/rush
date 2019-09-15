// Rust SHell
// v.0.1.0
// By Jacob Haig (jhaig343@gmail.com)

use structopt::StructOpt;
use std::path::PathBuf;
use std::path::Path;
use std::env;
use std::process::Command;
use std::process::ExitStatus;
use std::io::{self, Write, BufRead};


#[derive(StructOpt, Debug)]
#[structopt(name = "rush", about = "Rust Shell command structure.")]
struct Args {

	command: String, //inital cli command
	#[structopt(parse(from_os_str))]
	params: Vec<PathBuf> //command arguments(any number)
}




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

		let output = Command::new(execute).args(args).output().expect("Failed to execute command");
		
		if !output.status.success() {
			let err = output.stderr;
			print!("{}", String::from_utf8(err).ok().unwrap() );
		}
		else {
			let result = output.stdout;
			print!("{}", String::from_utf8(result).ok().unwrap() );
		}
		
	}
	
}