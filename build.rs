extern crate gcc;

use std::fs;

fn main() {
    let mut config = gcc::Config::new();

    let source_paths = fs::read_dir("libharu/src").unwrap()
        .map(|path| path.unwrap().path())
        .filter(|path| path.extension().unwrap() == "c");

    for source_path in source_paths {
        config.file(source_path);
    }

    config.include("libharu/include");
    config.compile("libharu.a");
}
