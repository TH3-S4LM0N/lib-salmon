# salmon::logger
a custom logger cause i dont like any other ones

### doc thing

#### usage
put 
```rust
use salmon::log::*;
```
at the top of any file you want to use it in.

#### logger struct
The `Logger` struct is to represent the logger, with various logging method implemented for it.
```rust
pub struct Logger<P>
where
    P: AsRef<Path>,
{
    pub logfile: P
}
```
`logfile` is the path to where you want it to log. To start the logger do something like:
```rust
let logger = log_rs::Logger::new("/home/salmon/logfile");
```
and then use `logger` to call the methods:
 - log \
    Just log a message
 - log_err \
    Log a message and panic
 - log_warn \
    Log a message as a warning

#### `Option<T>` and `Result<T, E>`
There is a `log` method for both of these meant to replace `.expect`ing everything.
```rust
fn log<P>(&self, msg: &str, logfile: &P) -> &T
where
    P: AsRef<Path>,
    E: fmt::Debug
```
`msg` is the message you want to be logged, logfile being where to log (usually just put `logger.logfile` to get the logfile from the struct). Then the behavior is the same as `.expect` but with logging.


