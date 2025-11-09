use colored::Colorize;
use mc_assembly::{MCAsmError, Mnemonic};
use std::env;
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;

fn show_compiled_mcfunction(assembly: &str) -> Result<String, String> {
    let result = mc_assembly::parse(&assembly);
    let show_ok_as_string = |arg: Vec<Mnemonic>| {
        arg.iter()
            .filter_map(|mnemonic| mnemonic.to_string().ok())
            .collect::<Vec<String>>()
            .join("\n")
    };
    let show_err_as_string = |e: Vec<(usize, MCAsmError)>| {
        e.iter()
            .map(|(index, error)| format!("An error occured at line {}: {:?}", index, error))
            .collect::<Vec<String>>()
            .join("\n")
    };
    result
        .map(|o| show_ok_as_string(o))
        .map_err(|e| show_err_as_string(e))
}

#[derive(Debug)]
struct EditorError;

impl std::fmt::Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The editor returned error code.")
    }
}

impl std::error::Error for EditorError {}

fn edit_using_tui_text_editor() -> Result<(String, PathBuf), Box<dyn Error>> {
    let found_editor = std::env::var("VISUAL").or(std::env::var("EDITOR"));
    let editor = match found_editor {
        Ok(o) => o,
        Err(_) => {
            println!(
                "{}: {}\n {} {}\n",
                "warning".yellow(),
                "both enviroment variables of VISUAL and EDITOR not set.",
                "-->".cyan(),
                "Vim will be used as editor automatically."
            );
            "vim".to_string()
        }
    };

    let temp = tempfile::NamedTempFile::new()?;

    // Running a text editor
    if !std::process::Command::new(editor)
        .arg(temp.path())
        .status()?
        .success()
    {
        Err(EditorError)?;
    };

    // Reading edited
    let mut buf = String::new();
    temp.reopen()?.read_to_string(&mut buf)?;
    Ok((buf, temp.path().to_path_buf()))
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (assembly, path) = if let Some(path) = args.get(1) {
        match std::fs::read_to_string(path) {
            Ok(assembly) => (assembly, PathBuf::from(path)),
            Err(e) => {
                println!(
                    "{}: failed to load the specified file.\nDetail: {}",
                    "error".red(),
                    e
                );
                return;
            }
        }
    } else {
        match edit_using_tui_text_editor() {
            Ok((assembly, path)) => (assembly, path),
            Err(e) => {
                println!(
                    "{}: failed to get edited string.\n\nCaused by:\n\t{}",
                    "error".red(),
                    e
                );
                return;
            }
        }
    };

    println!("{:>12} {}", "Compiling".green().bold(), path.display());

    match show_compiled_mcfunction(&assembly) {
        Ok(s) => {
            println!("{:>12} {}", "Finishing".green().bold(), path.display());
            println!("\n------ Compiled MCFunction ------\n\n{}", s);
        }
        Err(e) => {
            println!("{}", "Compilation failed\n".red().bold());
            println!("------ Errors------\n{}", e);
        }
    }
    println!();
}
