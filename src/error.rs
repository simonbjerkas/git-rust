use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("unsupported kind: {0}")]
    Usupported(String),
}
