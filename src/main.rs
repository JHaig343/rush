// Rust SHell
// v.0.1.0
// By Jacob Haig (jhaig343@gmail.com)

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "rush", about = "Rust Shell command structure.")]
struct Args {

	command: String, //inital cli command
	#[structopt(parse(from_os_str))]
	params: Vec<PathBuf> //command arguments(any number)
}


// shell entrypoint
fn main() -> Result<(), ExitFailure> {
	let args = Args::from_args();
	let command = args.command;
	let params = args.params;
	println!("{}", command);
	Ok(())
}