use std::process::Command;



pub fn shcmd(cmd: &str, args: &str) -> String {
    let shell = Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .arg(args)
                        .output()
                        .expect("Failed to run command!");
    String::from_utf8_lossy(&shell.stdout).to_string()
}