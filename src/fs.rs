macro_rules! append {
    ($f:expr, $c:expr) => {
        let mut file = OpenOptions::new()
                .append(true)
                .write(true)
                .create(true)
                .open($f)
                .expect("Failed to open $f with: append, write, and create");
        writeln!(file, $c);
    }
}