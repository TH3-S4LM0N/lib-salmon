use std::{
    io::{Result, Write},
    path::Path,
    fmt::Display,
    fs::OpenOptions
};

pub fn append<P>(file: P, contents: &str)
-> Result<()>
where
    P: AsRef<Path>
{
    let filer = OpenOptions::new()
                .append(true)
                .write(true)
                .create(true)
                .open(file);
    let mut file = match filer {
        Ok(f) => f,
        Err(e) => return Err(e)
    };


    let _ = match writeln!(file, "{contents}") {
        Ok(_) => (),
        Err(e) => return Err(e)
    };
    Ok(())
}