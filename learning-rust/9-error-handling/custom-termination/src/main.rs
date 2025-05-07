use std::{error::Error, fs::File};

enum MainReturn {
    MainOk(()),
    MainError(Box<dyn Error>),
}

impl std::process::Termination for MainReturn {
    fn report(self) -> std::process::ExitCode {
        match self {
            MainReturn::MainOk(()) => return ().report(),
            MainReturn::MainError(error) => std::process::ExitCode::FAILURE,
        }
    }
}

fn main() -> MainReturn {
    println!("Hello, world!");

    let x = File::open("Hello.txt");

    match x {
        Ok(_) => MainReturn::MainOk(()),
        Err(error) => MainReturn::MainError(Box::new(error)),
    }
}
