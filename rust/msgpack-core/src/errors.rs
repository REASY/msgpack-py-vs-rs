use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(Box<ErrorKind>);

#[derive(Error, Debug)]
#[error(transparent)]
pub enum ErrorKind {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("MsgPackDecodeError: {0}")]
    MsgPackDecodeError(#[from] rmpv::decode::Error),
    #[error("ItemMsgPackParser: {0}")]
    ItemMsgPackParser(String),
}

impl<E> From<E> for Error
where
    ErrorKind: From<E>,
{
    fn from(err: E) -> Self {
        Error(Box::new(ErrorKind::from(err)))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
