use std::fs;
use walkdir::WalkDir;

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
        "synctex.gz",
        "synctex.gz(busy)",
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

    for entry in WalkDir::new(folder_to_search) {
        let entry = entry.unwrap();
        // println!("{}", entry.path().display());
        // println!("entry.path().extension() = {:#?}", entry.path().extension());

        if let Some(ext) = entry.path().extension() {
            if forbidden.contains(&ext.to_str().unwrap()) {
                println!("Removing {}", entry.path().display());
                fs::remove_file(entry.path()).unwrap()
            }
        }
    }
}
