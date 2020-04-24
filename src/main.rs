// Rust SHell
// v.0.3.0
// By Jacob Haig (jhaig343@gmail.com)

use std::path::Path;
use std::env;
use std::process::{Command, Stdio};
mod utility;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {

	let mut redirect_flag : bool = false;
	let mut piping_flag : bool = false; 
	let mut rl = Editor::<()>::new();
	if rl.load_history("rush_history.txt").is_err() {
		println!("No previous history found.");
	}

	loop {
		let print_dir = env::current_dir();
		// NOTE: think this assert is superfluous
		assert!(print_dir.is_ok());

		let prompt = format!("\x1b[34m{}\x1b[0m=>$", print_dir.ok().unwrap().to_string_lossy());

		let line;

		let input = rl.readline(&prompt);
		match input {
			Ok(command) => {
				// Once a command is successfully executed, add it to readline history
				// (which can be accessed by pressing Up Arrow)
				rl.add_history_entry(command.as_str());
				line = command;
			},
			Err(ReadlineError::Interrupted) => { //Ctrl-C interrupts
				continue;
			},
			Err(err) => {
				println!("Unexpected error parsing input: {:?}", err);
				break;
			}
		}

		// separate string into words (split on spaces)
		let split = line.split(" ");
		let mut args = split.collect::<Vec<&str>>();
		let mut redirect_file: &str = "";
		let mut redirect_prog: &str = "";
		let mut redirect_args: Vec<&str> = Vec::new();
		if args.contains(&">") { //redirection
			redirect_flag = true;

			let ind = args.iter().position(|&r| r == ">").unwrap();
			args.remove(ind);
			// filename is the last argument in a "[command] > [file]" command.
			redirect_file = args.pop().unwrap();
		}
		else if args.contains(&"|") { //piping - [command/input] | [command]
			piping_flag = true;
			
			let ind = args.iter().position(|&r| r == "|").unwrap();
			args.remove(ind);
			let mut args_length = args.len() - ind;
			
			// last argument is command that input is being fed to; first command is output of another command
			// separate args past `|` as command + args being piped into
			while (args_length - 1) > 0 {
				redirect_args.push(args.pop().unwrap());
				args_length -= 1;
			}
			
			redirect_args.reverse();
			// last element to right of `|` is the command
			redirect_prog = args.pop().unwrap();
		}

		let execute = args.remove(0);

		if line == "exit" {
			// save readline history for future sessions
			rl.save_history("rush_history.txt").unwrap();
			break;
		}
		if execute == "cd" { //cd is a shell builtin, not a /bin program - so it needs to be created separately
			let root = Path::new(args[0]);
			if !(env::set_current_dir(&root).is_ok()) {
				let cd_err = env::set_current_dir(&root);
				
				utility::handle_err(cd_err, execute);
			}
			continue;
		}
		// changing stdio to piped when piping commands so that Child's output can be saved
		let output;
		if redirect_flag == true {
			output = Command::new(execute).args(args).stdout(Stdio::piped()).spawn();
		}
		else if piping_flag == true {
			output = Command::new(execute).args(args).stdout(Stdio::piped()).spawn();
		}
		else{
			output = Command::new(execute).args(args).spawn();
		}
		// Error message syntax: [COMMAND]: [Errormsg]
		// \x1b[Xm , where x is the ANSI color code colors following text output - 31 is red
		// 0 clears color code
		if output.is_err() {
			let failed_output = output.unwrap_err();
			println!("\x1b[31m{}: {}\x1b[0m", execute, failed_output );
			continue;
		}
		else {
			let success_output = output.expect("Shell failed to execute command.").wait_with_output();
			// Code below is for manually coloring terminal output - WIP
			// if execute == "ls" {
			// 	utility::test_ls_pretty_print(success_output);
			// 	continue;
			// }

			if redirect_flag == true {
				utility::redirect_to_file(success_output.unwrap(), redirect_file);
				redirect_flag = false;
				continue;
			}
			else if piping_flag == true {
				utility::pipe_to_program(success_output.unwrap(), redirect_prog, redirect_args);
				piping_flag = false;
				continue;
			}

			// this literally does fuck-all....
			// utility::pretty_print(success_output);
			

			
			
		}
	}
	
}