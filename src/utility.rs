use std::process::{Child, Command, Stdio, Output};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;

#[allow(dead_code)]
fn get_ls_colors() -> HashMap<&'static str, &'static str> {
    let mut key_values: HashMap<&str, &str> = HashMap::new();

        let colors: Vec<&str> = env!("LS_COLORS").split(":").collect();
        for obj in colors {
            let kvs: Vec<&str> = obj.split("=").collect();
            if kvs.len() <= 1 { //Skip incomplete key-value pairs
                continue;
            }
            key_values.insert(kvs[0], kvs[1]);

        }
        return key_values;
}

pub fn handle_err(error: std::result::Result<(), std::io::Error>, command: &str) {
    let failed_output = error.unwrap_err();
    println!("\x1b[31m{}: {}\x1b[0m", command, failed_output );
}

// ANSI escape codes used to print output in color
#[allow(dead_code)]
pub fn pretty_print(output: Child) {
    let content = output.wait_with_output().unwrap();
    if !content.status.success() {
        let err = content.stderr;
        print!("\x1b[31m{}\x1b[0m", String::from_utf8(err).ok().unwrap());
    }
    else{
        let result = content.stdout;
        // test_ls_pretty_print(&output);
        print!("{}", String::from_utf8(result).ok().unwrap());
    }
}

// Redirect stdout to a file when '>' is used in a command
pub fn redirect_to_file(output: Output, filename: &str) {
    let content = output;
    if !content.status.success() {
        let err = content.stderr;
        print!("\x1b[31m{}\x1b[0m", String::from_utf8(err).ok().unwrap());
    }
    else {
        let result = content.stdout;

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


// Testing out pretty-printing success output with 'ls'
// FIXME: with changes to spawn separate processes rather than running directly,
// color printing of filenames is no longer occurring. investigate
#[allow(dead_code)]
pub fn test_ls_pretty_print(output: Child) {

    let file_colors = get_ls_colors();
    let content = &output.wait_with_output().unwrap();

    let result = &content.stdout;
    let temp = String::from_utf8(result.to_vec()).ok().unwrap();
    let mut file_strings: Vec<String> = Vec::new();
    let mut output_result: Vec<String> = Vec::new();
    let files: Vec<&str>  = temp.split("\n").collect();
    for file in files {
        file_strings.insert(0, String::from(file));
    }
    for mut  file in file_strings {
        let  mut extension: &str = "";
        let file_ref = file.clone();

        let path  = Path::new(&file_ref);
        let potential_ext = path.extension().and_then(OsStr::to_str);
        if potential_ext != None {
            extension = potential_ext.unwrap();
        }

        for filetype in &file_colors {
            let color_ext = filetype.0.to_string();

            let ext_type = color_ext.trim_start_matches("*.");

            if ext_type == extension {
                let new_file = format!("\x1b[{}m{}\x1b[0m", filetype.1, file);
                file = new_file; 
            }
            
        }
        output_result.insert(0, file.clone());
    }
    for res in output_result {
        print!("{} ", res);
    }
}