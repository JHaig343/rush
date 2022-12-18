use std::process::{Command, Stdio, Output};
use std::fs::File;
use std::io::{BufWriter, Write};

use rustyline::completion::FilenameCompleter;
use rustyline::highlight::{ MatchingBracketHighlighter};
use rustyline::validate::MatchingBracketValidator;
use rustyline_derive::{Completer, Helper, Hinter, Highlighter, Validator};

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
pub struct TaskHelper {
    #[rustyline(Completer)]
    pub completer: FilenameCompleter,
    pub highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    pub validator: MatchingBracketValidator,
    pub colored_prompt: String,
}


pub fn handle_err(error: std::result::Result<(), std::io::Error>, command: &str) {
    let failed_output = error.unwrap_err();
    println!("\x1b[31m{}: {}\x1b[0m", command, failed_output );
}


// Redirect stdout to a file when '>' is used in a command
pub fn redirect_to_file(output: Output, filename: &str) {
    
    if !output.status.success() {
        let err = output.stderr;
        print!("\x1b[31m{}\x1b[0m", String::from_utf8(err).ok().unwrap());
    }
    else {
        let result = output.stdout;

        let file_output = String::from_utf8(result).ok().unwrap();
        
        let file = File::create(filename).expect("File creation failed unexpectedly");
        let mut out_writer = BufWriter::new(file);
        out_writer.write(file_output.as_bytes()).expect("File write failed unexpectedly");
    }
}

// Pipe shell program output as input to another shell program
// returns an Output object that either has its stdout printed or passed to file redirection
pub fn pipe_to_program(output: Output, program: &str, args: Vec<&str>) -> Option<Output> {
    let result = output.stdout;
    let command = Command::new(program).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn();

    if command.is_err() {
        let failed_output = command.unwrap_err();
        println!("\x1b[31m{}: {}\x1b[0m", program, failed_output);
        return None;
    }
    let mut command_result = command.ok().expect("Failed to start command");
    // put stuff in new scope so borrow ends
    let _ = {
        let stdin_stream = command_result.stdin.as_mut().expect("Couldn't get pipestream");
        stdin_stream.write_all(&result).ok().expect("Couldn't write to stream");
        stdin_stream.flush().ok().expect("Couldn't flush stream");
    };
    
    match command_result.wait_with_output() {
        Err(err) => panic!("couldn't write stdin to command: {:?}", err),
        Ok(out) => Some(out)
    }
}
