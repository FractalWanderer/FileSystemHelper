use std::path::PathBuf;
use clap::{ArgAction, Arg};
use clap::{Parser, Subcommand};

fn main() {

}

struct Cli{

}

#[derive(Subcommand)]
enum Command {

    #[clap(about = "Outputs the contents of a file to the console.")]
    OutputFileContents {
        #[clap(value_parser, short = 'p', long = "print")]
        file_path: String
    },
    #[clap(about = "Appends the given text to a file.")]
    AppendToFile {
        file_path: String,
        text: String
    },
    FindText {
        text: String
    },
    FindAndReplace {
        find: String,
        replace: String
    }
}