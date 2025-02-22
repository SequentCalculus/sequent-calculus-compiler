use crossbeam_channel::SendError;
use log::SetLoggerError;
use lsp_server::{ExtractError, Message, ProtocolError, Request};
use serde_json::Error as SerdeErr;
use std::{fmt, io::Error as IOErr};

#[derive(Debug)]
pub enum Error {
    IO(IOErr),
    UnsupportedMethod(String),
    Serde(SerdeErr),
    Protocol(ProtocolError),
    Log(SetLoggerError),
    Extract(ExtractError<Request>),
    Send(SendError<Message>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO error: {err}"),
            Error::UnsupportedMethod(method) => write!(f, "Method {method} is not supported"),
            Error::Serde(err) => write!(f, "Serde error: {err}"),
            Error::Protocol(err) => write!(f, "Protocol error: {err}"),
            Error::Log(err) => write!(f, "Logger error: {err}"),
            Error::Extract(err) => write!(f, "Error extracting args: {err}"),
            Error::Send(err) => write!(f, "Error sending message: {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<IOErr> for Error {
    fn from(err: IOErr) -> Error {
        Error::IO(err)
    }
}

impl From<SerdeErr> for Error {
    fn from(err: SerdeErr) -> Error {
        Error::Serde(err)
    }
}

impl From<ProtocolError> for Error {
    fn from(err: ProtocolError) -> Error {
        Error::Protocol(err)
    }
}

impl From<SetLoggerError> for Error {
    fn from(err: SetLoggerError) -> Error {
        Error::Log(err)
    }
}

impl From<ExtractError<Request>> for Error {
    fn from(err: ExtractError<Request>) -> Error {
        Error::Extract(err)
    }
}

impl From<SendError<Message>> for Error {
    fn from(err: SendError<Message>) -> Error {
        Error::Send(err)
    }
}
