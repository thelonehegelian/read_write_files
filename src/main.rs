use same_file::Handle;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let file = Path::new("text.txt");
    let std_handle = Handle::stdout().unwrap();
    let handle = Handle::from_path(&file).unwrap();

    if std_handle == handle {
        println!("stdout is the same as text.txt");
    } else {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        for (num, line) in reader.lines().enumerate() {
            println!("{}: {}", num, line.unwrap());
        }
    }
}
