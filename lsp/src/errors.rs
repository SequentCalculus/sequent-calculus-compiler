use crate::server::method::Method;
use crossbeam_channel::SendError;
use fun::{parser::result::ParseError, typing::errors::Error as TypeError};
use log::SetLoggerError;
use lsp_server::{
    ErrorCode, ExtractError, Message, Notification, ProtocolError, Request, RequestId, Response,
    ResponseError,
};
use lsp_types::{Diagnostic, Position, PublishDiagnosticsParams, Range, Uri};
use serde_json::Error as SerdeErr;
use std::{fmt, io::Error as IOErr};

#[derive(Debug)]
pub enum Error {
    UnsupportedMethod(String),
    InvalidPosition(Position),
    UndefinedIdentifier(String),
    BadRequest(Method, String),

    Parse(ParseError),
    Typing(TypeError),
    IO(IOErr),
    Serde(SerdeErr),
    Protocol(ProtocolError),
    Log(SetLoggerError),
    ExtractReq(ExtractError<Request>),
    ExtractNot(ExtractError<Notification>),
    Send(SendError<Message>),
}

impl Error {
    pub fn to_response(self, id: RequestId) -> Response {
        Response {
            id,
            result: None,
            error: Some(ResponseError {
                code: ErrorCode::RequestFailed as i32,
                message: self.to_string(),
                data: None,
            }),
        }
    }

    pub fn to_notification(self, uri: Uri) -> Notification {
        Notification {
            method: Method::PublishDiagnostics.to_string(),
            params: serde_json::to_value(PublishDiagnosticsParams {
                uri,
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 1,
                        },
                    },
                    message: self.to_string(),
                    ..Default::default()
                }],
                version: None,
            })
            .unwrap(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO error: {err}"),
            Error::UnsupportedMethod(method) => write!(f, "Method {method} is not supported"),
            Error::Serde(err) => write!(f, "Serde error: {err}"),
            Error::Protocol(err) => write!(f, "Protocol error: {err}"),
            Error::Log(err) => write!(f, "Logger error: {err}"),
            Error::ExtractReq(err) => write!(f, "Error extracting request args: {err}"),
            Error::ExtractNot(err) => write!(f, "Error extracting notification args: {err}"),
            Error::Send(err) => write!(f, "Error sending message: {err}"),
            Error::Parse(err) => write!(f, "Error while parsing program: {err}"),
            Error::Typing(err) => write!(f, "Error during typing: {err}"),
            Error::InvalidPosition(pos) => {
                write!(f, "Invalid source position: {},{}", pos.line, pos.character)
            }
            Error::UndefinedIdentifier(ident) => write!(f, "Undefined identifier {ident}"),
            Error::BadRequest(method, msg) => write!(f, "Bad request with method {method}: {msg}"),
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
        Error::ExtractReq(err)
    }
}

impl From<ExtractError<Notification>> for Error {
    fn from(err: ExtractError<Notification>) -> Error {
        Error::ExtractNot(err)
    }
}

impl From<SendError<Message>> for Error {
    fn from(err: SendError<Message>) -> Error {
        Error::Send(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::Parse(err)
    }
}

impl From<TypeError> for Error {
    fn from(err: TypeError) -> Error {
        Error::Typing(err)
    }
}
