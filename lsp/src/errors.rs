use crate::server::{document::Document, method::Method};
use crossbeam_channel::SendError;
use fun::{parser::result::ParseError, typing::errors::Error as TypeError};
use log::SetLoggerError;
use lsp_server::{
    ErrorCode, ExtractError, Message, Notification, ProtocolError, Request, RequestId, Response,
    ResponseError,
};
use lsp_types::{Diagnostic, Position, PublishDiagnosticsParams, Range, Uri};
use miette::SourceSpan;
use serde_json::Error as SerdeErr;
use std::{fmt, io::Error as IOErr};

#[derive(Debug)]
pub enum Error {
    UnsupportedMethod(String),
    InvalidPosition(Position),
    UndefinedIdentifier(String),
    BadRequest(Method, String),

    Parse {
        err: ParseError,
        loc: Option<(usize, usize)>,
    },
    Typing {
        err: TypeError,
        loc: (usize, usize),
    },
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

    pub fn to_notification(self, uri: Uri, doc: &Document) -> Notification {
        let pos = self.get_pos().unwrap_or_default();
        let start = doc.ind_to_pos(pos.0);
        let end = doc.ind_to_pos(pos.1);
        Notification {
            method: Method::PublishDiagnostics.to_string(),
            params: serde_json::to_value(PublishDiagnosticsParams {
                uri,
                diagnostics: vec![Diagnostic {
                    range: Range { start, end },
                    message: self.to_string(),
                    ..Default::default()
                }],
                version: None,
            })
            .unwrap(),
        }
    }

    fn get_pos(&self) -> Option<(usize, usize)> {
        match self {
            Error::Parse { loc, .. } => *loc,
            Error::Typing { loc, .. } => Some(*loc),
            _ => None,
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
            Error::Parse { err, .. } => write!(f, "Error while parsing program: {err}"),
            Error::Typing { err, .. } => write!(f, "Error during typing: {err}"),
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
        let loc = get_pos_parse(&err);
        Error::Parse { err, loc }
    }
}

impl From<TypeError> for Error {
    fn from(err: TypeError) -> Error {
        let loc = get_pos_type(&err);
        Error::Typing { err, loc }
    }
}

fn get_pos_parse(err: &ParseError) -> Option<(usize, usize)> {
    match err {
        ParseError::InvalidToken { location } => {
            let off = location.offset();
            Some((off, off))
        }
        ParseError::UnrecognizedEof { location, .. } => {
            let off = location.offset();
            Some((off, off))
        }
        ParseError::UnrecognizedToken { span, .. } => Some(span_to_pos(span)),
        ParseError::ExtraToken { span, .. } => Some(span_to_pos(span)),
        ParseError::User { .. } => None,
    }
}

fn get_pos_type(err: &TypeError) -> (usize, usize) {
    match err {
        TypeError::DefinedMultipleTimes { span, .. } => span_to_pos(span),
        TypeError::Undefined { span, .. } => span_to_pos(span),
        TypeError::Mismatch { span, .. } => span_to_pos(span),
        TypeError::UnboundVariable { span, .. } => span_to_pos(span),
        TypeError::WrongNumberOfArguments { span, .. } => span_to_pos(span),
        TypeError::ExpectedTermGotCovariable { span, .. } => span_to_pos(span),
        TypeError::ExpectedCovariableGotTerm { span, .. } => span_to_pos(span),
        TypeError::EmptyMatch { span } => span_to_pos(span),
        TypeError::MissingDtorInNew { span, .. } => span_to_pos(span),
        TypeError::ExpectedI64ForNew { span, .. } => span_to_pos(span),
        TypeError::ExpectedDataForNew { span, .. } => span_to_pos(span),
        TypeError::WrongNumberOfBinders { span, .. } => span_to_pos(span),
        TypeError::TypingContextMismatch { span, .. } => span_to_pos(span),
        TypeError::MissingCtorInCase { span, .. } => span_to_pos(span),
        TypeError::UnexpectedCtorsInCase { span, .. } => span_to_pos(span),
        TypeError::UnexpectedDtorsInNew { span, .. } => span_to_pos(span),
        TypeError::VarBoundMultipleTimes { span, .. } => span_to_pos(span),
        TypeError::CovarBoundMultipleTimes { span, .. } => span_to_pos(span),
        TypeError::TypeParameterBoundMultipleTimes { span, .. } => span_to_pos(span),
        TypeError::ExpectedI64ForConstructor { span, .. } => span_to_pos(span),
        TypeError::WrongNumberOfTypeArguments { span, .. } => span_to_pos(span),
        TypeError::UndefinedWrongTypeArguments { span, .. } => span_to_pos(span),
        TypeError::UnboundCovariable { span, .. } => span_to_pos(span),
    }
}

fn span_to_pos(span: &SourceSpan) -> (usize, usize) {
    let start = span.offset();
    let end = start + span.len();
    (start, end)
}
