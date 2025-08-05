use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError{
    #[error("Confy config error {0}")]
    ConfigError(#[from] confy::ConfyError),
}