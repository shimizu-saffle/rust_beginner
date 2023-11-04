use std::{fs, io};

fn main() -> io::Result<()> {
    let dir_entries = fs::read_dir(".")?;
    let iter_entries = dir_entries.map(|res| res.map(|e| e.path()));
    let mut vec_entries = iter_entries.collect::<Result<Vec<_>, io::Error>>()?;        

    vec_entries.sort();

    for entry in vec_entries  {
        println!("{}", entry.file_name().unwrap().to_str().unwrap())
    }

    Ok(())
}
