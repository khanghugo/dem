#[derive(Debug, thiserror::Error)]
pub enum DemoError {
    #[error("Failed to parse demo file")]
    ParseError,
    #[error("IOError: {source}")]
    IOError {
        #[from]
        source: std::io::Error,
    },
}
