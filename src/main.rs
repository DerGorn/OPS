use ops::{ErrorHandler, Res};
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path};

mod tokenizer;
use tokenizer::tokenizer;

mod parser;
use parser::Scope;

const FILE_EXTENSION: &str = "nop";

fn help_print(exec_path: &str) {
    println!(
        "
    Usage:
        {} source_file [OPTIONS]
    
    Available Options include:
        -h  --help       Prints this help message.
        -r  --run        Run the compiled executable.
    ",
        exec_path
    );
}

enum CommandLineArguments {
    HelpFlag,
    RunFlag,
}

impl CommandLineArguments {
    fn from_str(arg: &str) -> Res<CommandLineArguments> {
        match arg {
            x if ["-h", "--help"].contains(&x) => Ok(CommandLineArguments::HelpFlag),
            x if ["-r", "--run"].contains(&x) => Ok(CommandLineArguments::RunFlag),
            _ => Err(format!("{} is no valid option.", arg).into()),
        }
    }
}

struct CLI {
    exec_file: String,
    source_file: String,
    help: bool,
    run: bool,
}

impl CLI {
    fn new(exec_file: String) -> CLI {
        CLI {
            exec_file,
            source_file: String::from(""),
            help: false,
            run: false,
        }
    }

    fn from_argv(&mut self, err: &ErrorHandler) -> Res<()> {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() > 0 && args[0].contains(&self.exec_file) {
            args.drain(0..1);
        }
        if args.len() < 1 {
            return err.helpful_err("No source file specified.");
        }

        let source_file = Path::new(&args[0]);
        if !source_file.exists() {
            match CommandLineArguments::from_str(&args[0]) {
                Ok(CommandLineArguments::HelpFlag) => {
                    self.help = true;
                    return Ok(());
                }
                _ => {}
            }
            return ErrorHandler::err(&format!("Source file '{}' does not exist.", args[0]));
        }
        if source_file
            .extension()
            .and_then(|ext| {
                if ext == FILE_EXTENSION {
                    Some(true)
                } else {
                    None
                }
            })
            .is_none()
        {
            return ErrorHandler::err(&format!(
                "Source file '{}' is not a 'OPS' file. Please specify a valid '.{}' file.",
                args[0], FILE_EXTENSION
            ));
        }
        self.source_file = args[0].clone();

        let mut i = 1;
        while i < args.len() {
            let arg = match CommandLineArguments::from_str(&args[i]) {
                Ok(a) => a,
                Err(e) => return err.helpful_err(&e.to_string()),
            };
            match arg {
                CommandLineArguments::HelpFlag => self.help = true,
                CommandLineArguments::RunFlag => self.run = true,
            }
            i += 1;
        }
        Ok(())
    }
}

fn main() -> Res<()> {
    let exec_name = std::env::current_exe()?
        .file_name()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .ok_or_else(|| "Reading executable name failed.")?
        .to_string();
    let err = ErrorHandler::new(exec_name.clone());
    let mut args = CLI::new(exec_name.clone());
    args.from_argv(&err)?;
    if args.help {
        help_print(&exec_name);
        return Ok(());
    }
    let f = match File::open(&args.source_file) {
        Ok(f) => f,
        Err(e) => {
            return ErrorHandler::err(&format!(
                "Could not open source file '{}'. {}",
                &args.source_file, e
            ))
        }
    };
    let f = BufReader::new(f);
    let tokens = tokenizer(f, &args.source_file)?;
    // for token in tokens {
    //     println!(
    //         "{} Token with value: '{}' at {}",
    //         token.token.token_type(),
    //         token.token.to_string(),
    //         token.position
    //     )
    // }
    Scope::new().parse_block(tokens)?;
    Ok(())
}
