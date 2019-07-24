use std::env;
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
}

fn guess(n: i32) {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }

    // run with wrong arg to make me panic!
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap();
    println!("{}", arg);
    let n: i32 = arg.parse().unwrap();
    println!("{}", 2 * n);
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
