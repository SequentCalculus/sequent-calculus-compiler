use super::method::Method;
use crate::errors::Error;
use fun::{
    parser::parse_module,
    syntax::declarations::{Declaration, Module},
};
use log::info;
use lsp_server::{Message, Notification, Request, Response};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range};
use std::{fs::read_to_string, path::PathBuf};

pub struct MessageHandler {
    current_source: Option<String>,
    current_module: Option<Module>,
    source_path: Option<PathBuf>,
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {
            current_source: None,
            source_path: None,
            current_module: None,
        }
    }

    fn load_source(&mut self, path: PathBuf) -> Result<(), Error> {
        match self.source_path {
            Some(ref p) if *p == path => return Ok(()),
            _ => (),
        };
        let contents = read_to_string(&path)?;
        let parsed = parse_module(&contents)?;
        self.current_source = Some(contents);
        self.current_module = Some(parsed);
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

        info!("goto params: {params:?}");
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let path = uri.path();
        self.load_source(PathBuf::from(path.as_str()))?;
        let ident = self.get_ident(pos)?;
        info!("got identifier {ident}");
        let ind = self
            .current_module
            .as_ref()
            .unwrap()
            .declarations
            .iter()
            .find_map(|decl| match decl {
                Declaration::Data(data) => {
                    (data.name == ident).then_some(data.span.start().to_usize())
                }
                Declaration::Codata(cod) => {
                    (cod.name == ident).then_some(cod.span.start().to_usize())
                }
                Declaration::Def(df) => (df.name == ident).then_some(df.span.start().to_usize()),
            })
            .ok_or(Error::UndefinedIdentifier(ident))?;
        let pos = self.ind_to_pos(ind)?;
        info!("got position {pos:?}");
        //let prog = self.current_source.unwrap();

        let loc = Location::new(
            uri,
            Range {
                start: pos,
                end: Position {
                    line: pos.line,
                    character: pos.character + 1,
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

    fn get_ident(&self, pos: Position) -> Result<String, Error> {
        let source = self.current_source.clone().ok_or(Error::MissingSource)?;
        let line = source
            .lines()
            .nth(pos.line as usize)
            .ok_or(Error::InvalidPosition(pos))?;

        let mut following = line
            .chars()
            .nth(pos.character as usize)
            .ok_or(Error::InvalidPosition(pos))?;
        let mut end_pos = pos.character as usize;
        while following.is_alphanumeric() || following == '_' {
            end_pos += 1;
            if end_pos == line.len() {
                break;
            }
            following = line.chars().nth(end_pos).unwrap();
        }

        let mut prev = line
            .chars()
            .nth(pos.character as usize)
            .ok_or(Error::InvalidPosition(pos))?;
        let mut start_pos = pos.character as usize;
        while prev.is_alphanumeric() || following == '_' {
            start_pos -= 1;
            if start_pos == 0 {
                break;
            }
            prev = line.chars().nth(start_pos).unwrap();
        }
        if start_pos > 0 {
            start_pos += 1
        }

        let ident = &line[start_pos..end_pos];
        Ok(ident.to_owned())
    }

    fn ind_to_pos(&self, index: usize) -> Result<Position, Error> {
        let source = self.current_source.as_ref().ok_or(Error::MissingSource)?;
        let mut line = 0;
        let mut character = 0;
        for (ind, ch) in source.chars().enumerate() {
            if ind == index {
                break;
            }
            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }
        }
        Ok(Position { line, character })
    }
}
