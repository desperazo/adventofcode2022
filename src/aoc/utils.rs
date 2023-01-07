use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read(path: &str) -> Vec<String> {
    let mut store = Vec::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                store.push(ip);
            }
        }
    }
    store
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
