// Rust SHell
// v.0.1.0
// By Jacob Haig (jhaig343@gmail.com)

use structopt::StructOpt;
use std::path::PathBuf;
use std::process::Command;
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

		// for s in split {
		// 	println!("{}", s);
		// }

		let mut args = split.collect::<Vec<&str>>();
		let execute = args.remove(0);

		// println!("{}", line);
		if line == "exit" {
			break;
		}

		// println!("Work in progess!");
		let output = Command::new(execute).args(args).output().expect("Failed to execute command");
		let result = output.stdout;
		print!("{}", String::from_utf8(result).ok().unwrap() );
	}
	
}




// let output = Command::new("echo")
 //                     .arg("Hello blorld")
 //                     .output()
 //                     .expect("Failed to execute command");
	// let string = output.stdout;
	// println!("{}", String::from_utf8(string).ok().unwrap());
// shell entrypoint
// fn main() -> Result<(), ExitFailure> {
// 	let args = Args::from_args();
// 	let command = args.command;
// 	let params = args.params;
// 	println!("{}", command);
// 	// for i in params {
// 	// 	// need to use to_string_lossy as PathBuf isn't formatted
// 	// 	println!("{}", i.to_string_lossy());
// 	// }
	
// 	let output = Command::new("echo")
//                      .arg("Hello blorld")
//                      .output()
//                      .expect("Failed to execute command");
// 	println!("{}",String::from_utf8(output.stdout.as_slice()) );
// 	Ok(())
// }