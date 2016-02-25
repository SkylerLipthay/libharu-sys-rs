extern crate gcc;

use std::fs;

fn main() {
    let mut config = gcc::Config::new();

    config.define("LIBHPDF_HAVE_NOZLIB", None);
    config.define("LIBHPDF_HAVE_NOPNGLIB", None);

    let source_paths = fs::read_dir("libharu/src").unwrap()
        .map(|path| path.unwrap().path())
        .filter(|path| path.extension().unwrap() == "c");

    for source_path in source_paths {
        config.file(source_path);
    }

    config.include("libharu/include");
    config.compile("libharu.a");
}
