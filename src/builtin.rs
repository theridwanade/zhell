pub enum Builtin {
    Exit,
}

impl Builtin {
    pub fn parse(cmd: &str) -> Option<Self> {
        match cmd {
            "exit" => Some(Builtin::Exit),
            _ => None,
        }
    }

    pub fn execute(&self) {
        match self {
            Builtin::Exit => std::process::exit(0),
        }
    }
}