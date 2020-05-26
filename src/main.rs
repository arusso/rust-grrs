use std::fmt::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::result::Result;

use structopt::StructOpt;
use exitfailure::ExitFailure;
use failure::ResultExt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), ExitFailure>  {
    let args = Cli::from_args();

    let f = File::open(&args.path.as_path())
        .with_context ( |_| format!("Error reading `{}`", &args.path.to_str().unwrap()))?;

    let reader = BufReader::new(f);
    find_matches(reader.lines(), &args.pattern, &mut std::io::stdout());


    Ok(())
}

 fn find_matches<B: BufRead> (content: Lines<B>, pattern: &str, mut writer: impl std::io::Write) {
    for line in content {
        let line = line.unwrap();
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
 }
