use std::env;
use std::num::ParseIntError;
fn main() {
    // change input to make me panic!
    guess(9);

    let file_name = "foo.rs";
    match extention_explicit(file_name) {
        None => println!("No file extension found."),
        Some(res) => println!("File extension: {}", res),
    }
    let file_name1 = "bar.js";
    match extention_explicit_map(file_name1) {
        None => println!("No file extension found."),
        Some(res) => println!("File extension: {}", res),
    }

    assert_eq!(extention_explicit("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(extention_explicit_map("foobar").unwrap_or("rs"), "rs");

    let num = "k10";

    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn guess(n: i32) {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }

    // run with wrong arg to make me panic!
        match double_arg(env::args()) {
                    Ok(n) => println!("{}", n),
                            Err(err) => println!("Error: {}", err),
                                }

}

fn find(word: &str, needle: char) -> Option<usize> {
    for (offset, c) in word.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn extention_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i + 1..]),
    }
}

fn extention_explicit_map(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i + 1..])
}

fn double_number(number: &str) -> Result<i32, ParseIntError> {
    number.parse::<i32>().map(|num| 2 * num)
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
        argv.nth(1)
                    .ok_or("Please give at least one argument".to_owned())
                            .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
}
