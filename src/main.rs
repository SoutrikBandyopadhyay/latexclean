use clap::Parser;
use std::fs;
use walkdir::WalkDir;

/// This program cleans the directory it runs on for any auxiliary files that latex generates.
/// The program recursively searches the directory for some known latex extensions and deletes
/// them. To list all the extensions that would be deleted, run the program with -l or --list flag.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Flag to enable verbose output
    #[clap(long, short)]
    verbose: bool,

    ///Flag to list all the extensions that would be deleted
    #[clap(long, short)]
    list: bool,
}
fn main() {
    let folder_to_search = ".";
    let forbidden = [
        "aux",
        "lof",
        "log",
        "lot",
        "fls",
        "out",
        "toc",
        "fmt",
        "fot",
        "cb ",
        "cb3",
        "lb",
        "bbl",
        "bcf",
        "blg",
        "xml",
        "fdb_latexmk",
        "synctex",
        "synctex(busy)",
        "pdfsync",
        "alg",
        "loa",
        "nav",
        "pre",
        "snm",
        "vrb",
        "acn",
        "acr",
        "glg",
        "glo",
        "gls",
        "glsdefs",
        "lzo",
        "lzs",
        "slg",
        "slo",
        "sls",
        "nlg",
        "nlo",
        "nls",
    ];

    let cli = Args::parse();

    // If the user supplied the list flag in the arguments,
    // List all the extensions to be deleted
    if cli.list {
        for i in forbidden {
            println!("{}", i);
        }
    } else {
        //Using the WalDir crate to recursively search the files
        for entry in WalkDir::new(folder_to_search) {
            let entry = entry.unwrap();

            if let Some(ext) = entry.path().extension() {
                if forbidden.contains(&ext.to_str().unwrap()) {
                    // If the user supplied the verbose flag in the arguments,
                    // Print all the files that are removed
                    if cli.verbose {
                        println!("Removing {}", entry.path().display());
                    }
                    fs::remove_file(entry.path()).unwrap()
                }
            }
        }
    }
}
