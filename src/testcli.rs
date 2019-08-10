// structopt is useful for building CLI applications
use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;


#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}



impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "file: {}", self.path.to_string_lossy())
    }
}

// fn main() -> Result<(), ExitFailure> {
//     let args = Cli::from_args();
//     // read in the file contents (as one big string...)
//     // question mark on end acts like commented match block below(easy way to catch errors)
//     let content = std::fs::read_to_string(&args.path).with_context(|_| format!("Error reading {}", &args.path.to_string_lossy()))?;
//     // let result = match content {
//         // Ok(_content) => { _content },
//         // Err(error) => { panic!("An Error occurred: {}", error);}
//     // };
//     println!("file content: {}", content );
//     Ok(())
//     // println!("{:?}", content);
//     // echo back line with the pattern being searched for
//     // FIXME: this doesn't gel with new Result content type.
//     // for line in content.lines() {
//     //     if line.contains(&args.pattern) {
//     //         println!("{}", line);
//     //     }
//     // }
// }
