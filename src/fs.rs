use std::{
    io::{Result, Write},
    path::Path,
    fmt::Display,
    fs::OpenOptions
};

pub fn append<P, S>(file: P, contents: S)
-> Result<()>
where
    P: AsRef<Path>,
    S: AsRef<String> + Display
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