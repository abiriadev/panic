use std::{
    error::Error,
    fs::{self, read_to_string, File},
    io::{self, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    let v = vec![1, 2, 3, 4, 5, 6];

    // v[99];

    resolve_file_old();

    resolve_file_new();

    let filename = "hello.txt";

    let username = read_user_name_from_file(filename).unwrap();

    println!("{}", username);

    let username = match read_user_name_from_file("hel1lo.txt") {
        Ok(data) => println!("{}", data),
        Err(err) => println!("{}", err),
    };

    let my_name = read_user_name_from_file_by_question_token(filename).unwrap();

    println!("{}", my_name);

    // shortest_way
    let data: String = fs::read_to_string(filename).unwrap();

    println!("{}", data);

    let filedata = File::open(filename)?;

    println!("{:?}", filedata);

    parseIP();

    guessing_game();

    Ok(())
}

// loop {
// // let guess: i32 = match guess. {}
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("please input value in rnage 1 to 100");
        }

        Guess { value }
    }

    pub fn getValue(&self) -> i32 {
        self.value
    }
}

fn guessing_game() {
    let secret_number = 50;

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        // input.trim();

        if input.trim().eq("quit") {
            break;
        }

        let guess = Guess::new(match input.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!("plz input numbers"),
        });

        match guess.getValue().cmp(&secret_number) {
            Ordering::Less => println!("LESS!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            Ordering::Greater => println!("GREATER!"),
        }
    }

    println!("good bye :)");
}

use std::cmp::Ordering;
use std::net::IpAddr;

fn parseIP() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("{}", home);
}

fn read_user_name_from_file_by_question_token(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_user_name_from_file(filename: &str) -> Result<String, io::Error> {
    let mut f: Result<File, io::Error> = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn resolve_file_new() {
    const FILE_NAME: &str = "data.log";

    if let Err(err) = File::open(FILE_NAME) {
        if let ErrorKind::NotFound = err.kind() {
            File::create(FILE_NAME).unwrap();
        } else {
            panic!("could not opened file: {:?}", FILE_NAME)
        }
    }
}

fn resolve_file_old() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(data) => data,
        Err(ref e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("could not created file: {}", e);
                }
            },
            other_error => panic!("could not opened file: {:?}", other_error),
        },
    };
}

enum myResult<T, E> {
    Ok(T),
    Err(E),
}
