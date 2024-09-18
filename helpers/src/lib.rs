pub mod trie;

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
