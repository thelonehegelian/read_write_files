use memmap::Mmap;
use same_file::Handle;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    /*************************************************
     * PREVENTS READING AND WRITING FROM THE SAME FILE
     *************************************************/

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

    /*******************************
     * OPEN FILE USING A MEMORY MAP
     ******************************/

    let file = File::open("text.txt").unwrap();
    // The Mmap::map function assumes the file behind
    // the memory map is not being modified at the same
    // time by another process or else a race condition occurs.
    let map = unsafe { Mmap::map(&file).unwrap() };
    print!("{:?}", map);
}
