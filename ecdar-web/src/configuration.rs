use std::{env::args, process::exit};

#[derive(Debug)]
pub struct Configuration {
    pub root: String,
    pub serve: String,
}

impl Configuration {
    pub fn create() -> Self {
        let mut new = Self::default();
        let mut args = args();
        args.next().expect("Not running a program");

        loop {
            let option = match args.next() {
                Some(option) => option,
                None => break,
            };

            match option.as_str() {
                "--root" | "-r" => {
                    let root = args.next().expect(r#"Missing a folder after "root""#);
                    new.root = root;
                }

                "--serve" | "-s" => {
                    let ip = args.next().expect(r#"Missing an ip address after "serve"#);
                    new.serve = ip;
                }

                "--help" | "-h" | _ => {
                    println!("{}", include_str!("./help.txt"));
                    exit(0);
                }
            }
        }

        new
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            root: ".".into(),
            serve: "0.0.0.0:3000".into(),
        }
    }
}
