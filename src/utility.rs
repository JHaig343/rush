use std::process::Child;
use std::process::Output;
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
// FIXME: IO is being delayed somehow until after next command is run, then fails with "file not found" error
pub fn redirect_to_file(output: Output, filename: &str) {
    // print!("{}", filename);
    let content = output;
    if !content.status.success() {
        let err = content.stderr;
        print!("\x1b[31m{}\x1b[0m", String::from_utf8(err).ok().unwrap());
    }
    else {
        let result = content.stdout;
        // print!("redirect test: {}", String::from_utf8(result).ok().unwrap());
        // print!("redirect test: {}", String::from_utf8(result).ok().unwrap());
        let file_output = String::from_utf8(result).ok().unwrap();
        
        let file = File::create(filename).expect("done f'ed up");
        // // fs::write(filename, file_output).expect("Unable to write to file\n");
        let mut out_writer = BufWriter::new(file);
        out_writer.write(file_output.as_bytes()).expect("f'ed up again");
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