use super::method::Method;
use crate::errors::Error;
use log::info;
use lsp_server::{Message, Notification, Request, Response};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range};
use std::{fs::read_to_string, path::PathBuf};

pub struct MessageHandler {
    current_source: Option<String>,
    source_path: Option<PathBuf>,
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {
            current_source: None,
            source_path: None,
        }
    }

    fn load_source(&mut self, path: PathBuf) -> Result<(), Error> {
        match self.source_path {
            Some(ref p) if *p == path => return Ok(()),
            _ => (),
        };
        let contents = read_to_string(&path)?;
        info!("loaded source {contents}");
        self.current_source = Some(contents);
        self.source_path = Some(path);
        Ok(())
    }

    pub fn handle_message(&mut self, msg: Message) -> Result<Response, Error> {
        match msg {
            Message::Request(req) => self.handle_request(req),
            Message::Response(resp) => self.handle_response(resp),
            Message::Notification(not) => self.handle_notification(not),
        }
    }

    fn handle_request(&mut self, req: Request) -> Result<Response, Error> {
        info!("got request with type {}", req.method);
        let method = req.method.parse::<Method>()?;
        match method {
            Method::GotoDefinition => self.goto_definition(req),
        }
    }

    fn handle_response(&self, resp: Response) -> Result<Response, Error> {
        info!("got response: {resp:?}");
        Err(Error::UnsupportedMethod("Response".to_owned()))
    }

    fn handle_notification(&self, not: Notification) -> Result<Response, Error> {
        info!("got notification: {not:?}");
        Err(Error::UnsupportedMethod(not.method))
    }

    fn goto_definition(&mut self, req: Request) -> Result<Response, Error> {
        let (id, params) =
            req.extract::<GotoDefinitionParams>(&Method::GotoDefinition.to_string())?;
        let uri = params.text_document_position_params.text_document.uri;
        let path = uri.path();
        self.load_source(PathBuf::from(path.as_str()))?;

        let loc = Location::new(
            uri,
            Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 1,
                },
            },
        );
        let result = Some(GotoDefinitionResponse::Scalar(loc));
        let result = serde_json::to_value(&result)?;
        let resp = Response {
            id,
            result: Some(result),
            error: None,
        };
        Ok(resp)
    }
}
