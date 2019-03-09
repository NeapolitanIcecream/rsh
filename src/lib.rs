use std::env;
use std::error::Error;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;

pub struct Args<'a> {
    args: Vec<&'a str>,
    arg_num: usize,
    arg_iter: usize,
}

impl<'a> Args<'a> {
    pub fn new(input: &String) -> Args {
        let args = Args::divide_arguments(input);

        let arg_num = args.len();

        let arg_iter = 0;

        return Args { args, arg_num, arg_iter };
    }

    pub fn deal(self) -> Result<(), &'static str> {
        let mut iter = self.args.iter();

        iter.next();

        match self.args[self.arg_iter] {
            "cd" => { return self.deal_cd(); }
            "exit" => { return self.deal_exit(); }
            cmd => {
                if let Err(err) = Command::new(cmd).args(iter).status() {
                    eprintln!("{}", err.description());
                    return Err("Failed to apply command.");
                }
            }
        }

        return Ok(());
    }

    fn deal_cd(self) -> Result<(), &'static str> {
        if self.arg_num > 2 {
            return Err("Expect no more than 2 arguments.");
        }

        if self.arg_num == 1 {
            let home = match env::var("HOME") {
                Ok(dir) => dir,
                Err(_) => {
                    return Err("Got an error when trying to read home_dir.");
                }
            };

            return match env::set_current_dir(home) {
                Ok(_) => Ok(()),
                Err(_) => Err("Got an error when trying to set current dir to home."),
            };
        }

        let dir = Path::new(self.args[1]);

        return match env::set_current_dir(dir) {
            Ok(_) => Ok(()),
            Err(_) => Err("Got an error when trying to set current dir."),
        };
    }

    fn deal_exit(self) -> Result<(), &'static str> {
        if self.arg_num > 1 {
            return Err("Expect no more than 1 arguments.");
        }

        process::exit(1);
    }

    fn divide_arguments(input: &String) -> Vec<&str> {
        return input.trim().split_whitespace().collect();
    }
}

pub fn prompt() -> Result<(), &'static str> {
    let mut out = io::stdout();

    let home = match env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => {
            return Err("Got an error when trying to read home_dir.");
        }
    };

    let home = home.as_str();

    let path = match pwd() {
        Ok(pwd) => pwd,
        Err(e) => {
            return Err(e);
        }
    }.replace(home, "~");

    println!("{}", path);

    if let Err(_) = out.write("> ".as_bytes()) {
        return Err("Got an error when trying to write prompt to stdout.");
    };

    if let Err(_) = out.flush() {
        return Err("Got an error when trying to flush stdout.");
    };

    return Ok(());
}

fn pwd() -> Result<String, &'static str> {
    let dir = match env::current_dir() {
        Ok(path_buf) => path_buf,
        Err(_) => {
            return Err("Got an error when trying to get current dir.");
        }
    };

    let path = dir.as_path().display().to_string();

    return Ok(path);
}