use std::{env, fs};
use std::path::PathBuf;
use walkdir::WalkDir;
use clap::{Parser, Subcommand};
use std::cmp;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

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

                let file_contents = fs::read_to_string(path_buf).expect("Could not read the file to a string!");

                print!("\n{}", file_contents);
            }
            Commands::AppendToFile { file_name, text } => {
                let path_buf = get_file(file_name);

                let previous_text = fs::read_to_string(&path_buf).expect("Could not read the file to a string!");

                let new_text = previous_text + text;

                fs::write(path_buf, new_text).expect("Failed to write the contents provided to the file.");
            }
            Commands::FindText { text, context_size, no_highlight } => {

                let file_paths = get_file_paths();
                let progress_bar = create_progress_bar(file_paths.len() as u64);

                let search_results: Vec<SearchResult> = Vec::new();

                for (index, file) in file_paths.iter().enumerate() {

                    progress_bar.set_position(index as u64);

                    let contents = fs::read_to_string(&file);

                    match contents {
                        Ok(contents) => {

                            let sliced_contents: Vec<&str> = contents.split("\n").collect();
                            let mut context_windows: Vec<ContextWindow> = Vec::new();
                            let mut occurences = 0;

                            for(line_index, line) in sliced_contents.iter().enumerate() {
                                if line.contains(text) {
                                    occurences += 1;


                                }
                            }

                       }
                        // This suppresses any file authorization errors, among other errors that might occur.
                        Err(_) => continue
                    }
                }

            }
            Commands::FindAndReplace { find, replace} => {

            }
        }
    }
}

struct SearchResult {
    file_name: String,
    file_path: String,
    number_of_occurrences: usize,
    context_window: ContextWindow
}

struct ContextWindow {
    window_contents: Vec<String>
}

fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
                     .expect("Unable to set the progress bar template."));

    return pb;
}

fn print_context_window(window_contents: &[&str], file_name: &str, text: &str, no_highlight: bool) {

    if no_highlight {
        println!("\nText '{}' found in file: {}\n", text, file_name);
    } else {
        println!("\nText '{}' found in file: {}\n", text.red(), file_name.green());
    }

    println!("---");

    for line in window_contents {
        if no_highlight {
            println!("{}", line);
        } else {
            println!("{}", highlight_text(line, text, Color::Red));
        }
    }

    println!("---\n");
}

fn highlight_text(full_text: &str, text_to_highlight: &str, color: Color) -> String {

    let replacement = text_to_highlight.color(color).to_string();

    return full_text.replace(text_to_highlight, &replacement);
}

fn get_file_paths() -> Vec<PathBuf> {

    let current_directory = env::current_dir().expect("Failed to get the current working directory.");

    let mut paths: Vec<PathBuf> = Vec::new();

    for directory_entry in WalkDir::new(current_directory){

        match directory_entry {
            Ok(entry) => {
                paths.push(entry.into_path())
            }
            Err(e) => {
                eprintln!("An error occurred {}", e)
            }
        }
    }

    return paths;
}

fn get_file(file_name: &String) -> PathBuf {

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

        if path.file_name().map(|os| os.to_str().expect("Could not parse the OsStr to a string.") == file_name).unwrap(){
            return path.to_path_buf();
        }

    }

    panic!("No file found with the name: {}", file_name)
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "print", about = "Outputs the contents of a file to the console.")]
    OutputFileContents {
        #[clap(value_parser)]
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
        text: String,
        #[clap(value_parser, default_value_t = 3)]
        context_size: u8,
        #[clap(long = "no-highlight", action = clap::ArgAction::SetTrue)]
        no_highlight: bool
    },
    #[clap(name = "replace", about = "Finds the given text and replaces it with the given text.")]
    FindAndReplace {
        #[clap(value_parser)]
        find: String,
        #[clap(value_parser)]
        replace: String
    }
}