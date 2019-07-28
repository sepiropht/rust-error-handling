use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::num;
use std::path::Path;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}
impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError {
                    CliError::Io(err)
                            }
}

impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError {
                    CliError::Parse(err)
                            }
}

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>().map(|n| 2 * n)?;
    Ok(2 * n)
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
