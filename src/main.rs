// structopt is useful for building CLI applications
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);



impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "file: {} args: {}", self.path.to_string_lossy(), self.pattern)
    }
}

fn main() -> Result<(), CustomError> {
    let args = Cli::from_args();
    println!("CLI Info: {}", args);
    // read in the file contents (as one big string...)
    // question mark on end acts like commented match block below
    let content = std::fs::read_to_string(&args.path).map_err(|err| CustomError(format!("Error reading {}: {}", &args.path.to_string_lossy(), err)))?;
    // let result = match content {
        // Ok(_content) => { _content },
        // Err(error) => { panic!("An Error occurred: {}", error);}
    // };
    println!("file content: {}", content );
    Ok(())
    // println!("{:?}", content);
    // echo back line with the pattern being searched for
    // FIXME: this doesn't gel with new Result content type.
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}
