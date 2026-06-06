pub enum Builtin {
    Exit,
    Echo,
}

impl Builtin {
    pub fn parse(cmd: &str) -> Option<Self> {
        match cmd {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo),
            _ => None,
        }
    }

    pub fn execute(&self, args: &str) {
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => println!("{}", args)
        }
    }
}