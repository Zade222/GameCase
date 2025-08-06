use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError{
    #[error("I/O Error")] 
    Io(#[from] io::Error), 

    #[error("Confy config error {0}")]
    ConfigError(#[from] confy::ConfyError),
}