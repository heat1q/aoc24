use std::fs;
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let path = Path::new("./inputs").join(filename);
    fs::read_to_string(path)
        .expect("read input file")
        .lines()
        .map(Into::into)
        .collect()
}
