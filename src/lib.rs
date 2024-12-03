use std::fs;
use std::path::Path;

pub fn read_file(filename: impl AsRef<Path>) -> String {
    let path = Path::new("./inputs").join(filename);
    fs::read_to_string(path).expect("read input file")
}

pub fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    read_file(filename).lines().map(Into::into).collect()
}
