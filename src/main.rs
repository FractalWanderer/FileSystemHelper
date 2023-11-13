use std::{env, fs};
use std::path::PathBuf;
use clap::{ArgAction, Arg};
use walkdir::WalkDir;
use clap::{Parser, Subcommand};

fn main() {
    let file_handler = FileHandler::parse();

    file_handler.command.execute();
}

#[derive(Parser)]
#[clap(name = "fh")]
#[clap(name = "FileSystemHelper", about = "File System Helper designed to assist in common file system interactions.")]
struct FileHandler {
    #[clap(subcommand)]
    command: Commands
}

trait CommandTrait {
    fn execute(&self);
}

impl CommandTrait for Commands {
    fn execute(&self) {
        match &self {
            Commands::OutputFileContents { file_name} => {
                let path_buf = get_file(file_name);

                match path_buf {
                    None => {
                        println!("No file found matching the given name.");
                    }
                    Some(path_buf) => {

                        let file_contents = fs::read_to_string(path_buf)
                            .expect("Could not read the file to a string!");

                        print!("\n{}", file_contents)
                    }
                }

            }
            Commands::AppendToFile { file_name, text } => {

            }
            Commands::FindText { text } => {

            }
            Commands::FindAndReplace { find, replace} => {

            }
        }
    }
}

fn get_file(file_name: &String) -> Option<PathBuf> {

    let current_directory = env::current_dir()
        .expect("Failed to get the current working directory.");

    for directory_entry in WalkDir::new(current_directory) {
        let directory_entry = match directory_entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let path = directory_entry.path();

        if path.file_name().map_or(false, |os| os.to_str().expect("Could not parse the OsStr to a string.") == file_name){
            return Some(path.to_path_buf());
        }

    }

    return None;
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "print", about = "Outputs the contents of a file to the console.")]
    OutputFileContents {
        #[clap(value_parser, short = 'p', long = "print")]
        file_name: String
    },
    #[clap(name = "append", about = "Appends the given text to a file.")]
    AppendToFile {
        #[clap(value_parser)]
        file_name: String,
        #[clap(value_parser)]
        text: String
    },
    #[clap(name = "find", about = "Searches recursively in the current directory and finds all files containing the given text.")]
    FindText {
        #[clap(value_parser)]
        text: String
    },
    #[clap(name = "replace", about = "Finds the given text and replaces it with the given text.")]
    FindAndReplace {
        #[clap(value_parser)]
        find: String,
        #[clap(value_parser)]
        replace: String
    }
}