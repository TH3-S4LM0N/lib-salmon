//! Very simple logging library with the goal of being a drop in replacement for ``.expect``.
//! Please do not use this, I made this for me and it will change for whatever I need.

use std::{path::Path, fs::{File, write}, io::Write, fmt};
use chrono::Local;

/// Structure representing the logger.
pub struct Logger<P>
where P: AsRef<Path>
{
    pub logfile: P
}

impl<P> Logger<P>
where P: AsRef<Path>
{
    /// Create a new logger
    pub fn new(logfile: P) -> Logger<P> {
        let mut file = File::create(&logfile).expect("Failed to create logfile");
        let log_msg = format!("{} - {}", Local::now().format("%H:%M:%S"), "Initialized logger");
        file.write_all(log_msg.as_bytes()).expect("Failed to write logfile");
        
        return Logger {
            logfile: logfile
        };
    }
    /// Log ``msg`` at the logfile value passed to ``new``.
    pub fn log(&self, msg: &str) {log(msg, &self.logfile, false)}
    /// Log ``msg`` and err if there is one at the logfile passed to ``new`` and panic.
    pub fn log_err(&self, msg: &str) {log_err(msg, &self.logfile, None)}
    /// Log ``msg`` at the logfile passed to ``new`` as a warning.
    pub fn log_warn(&self, msg:&str) {log(msg, &self.logfile, true)}
}

fn log_err<P>(msg: &str, logfile: &P, err: Option<&dyn fmt::Debug>) -> !
where P: AsRef<Path>
{
    let log_msg = format!("{} - ERROR: {:?} - {}\n", Local::now().format("%H:%M:%S"), err, msg);

    write(logfile, &log_msg).expect("Failed to write logfile");

    panic!("{msg}");
}

fn log<P>(msg: &str, logfile: &P, warning: bool)
where P: AsRef<Path>
{
    let log_msg: String;
    if warning {
        log_msg = format!("{} - WARNING: {}", Local::now().format("%H:%M:%S"), msg);
    } else {
        log_msg = format!("{} - {}", Local::now().format("%H:%M:%S"), msg);
    }

    write(logfile, &log_msg).expect("Failed to write logfile");
}

/// Trait to impl a ``log`` method for ``Result<T, E>``.
pub trait ResultLog<T, E> {
    /// Same as the ``expect`` method for ``Result<T, E>`` but it logs ``msg`` instead of printing it
    fn log<P>(&self, msg: &str, logfile: &P) -> &T
    where P: AsRef<Path>,
          E: fmt::Debug;
}
impl<T, E> ResultLog<T, E> for Result<T, E> {
    /// Same as the ``expect`` method for ``Result<T, E>`` but it logs ``msg`` instead of printing it
    fn log<P>(&self, msg: &str, logfile: &P) -> &T
    where P: AsRef<Path>,
          E: fmt::Debug
    {
        match self {
            Ok(v) => v,
            Err(e) => log_err(msg, &logfile, Some(&e))
        }
    }
}

/// Trait to impl a ``log`` method for ``Option<T>``.
pub trait OptionLog<T> {
    /// Same as the ``expect`` method for ``Option<T>`` but it logs ``msg`` instead of printing it
    fn log<P>(&self, msg: &str, logfile: &P) -> &T
    where P: AsRef<Path>;
}
impl<T> OptionLog<T> for Option<T> {
    /// Same as the ``expect`` method for ``Option<T>`` but it logs ``msg`` instead of printing it
    fn log<P>(&self, msg: &str, logfile: &P) -> &T
    where P: AsRef<Path>
    {
        match self {
            Some(v) => v,
            None => log_err(msg, logfile, None)
        }
    }
}


