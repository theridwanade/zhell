use crate::utils::find_in_path;

pub enum Builtin {
    Exit,
    Echo,
    Type,
}

impl Builtin {
    pub fn parse(cmd: &str) -> Option<Self> {
        match cmd {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo),
            "type" => Some(Builtin::Type),
            _ => None,
        }
    }

    pub fn execute(&self, args: &str) {
        match self {
            Builtin::Exit => std::process::exit(0),
            Builtin::Echo => println!("{}", args),
            Builtin::Type => {
                if Builtin::parse(args).is_some() {
                    println!("{} is shell builtin", args);
                } else if let Some(path) = find_in_path(args) {
                    println!("{} is {}", args, path.display());
                } else {
                    println!("{}: not found", args);
                }
            }
        }
    }
}
