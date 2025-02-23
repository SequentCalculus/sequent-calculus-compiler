use super::{document::Document, method::Method};
use crate::errors::Error;
use log::info;
use lsp_server::{Message, Notification, Request, Response};
use lsp_types::{DidOpenTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse};

pub struct MessageHandler {
    doc: Document,
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {
            doc: Document::new(),
        }
    }

    pub fn handle_message(&mut self, msg: Message) -> Result<Option<Response>, Error> {
        match msg {
            Message::Request(req) => self.handle_request(req).map(Some),
            Message::Response(resp) => self.handle_response(resp).map(|_| None),
            Message::Notification(not) => self.handle_notification(not).map(|_| None),
        }
    }

    fn handle_request(&mut self, req: Request) -> Result<Response, Error> {
        info!("got request with type {}", req.method);
        let method = req.method.parse::<Method>()?;
        match method {
            Method::GotoDefinition => self.goto_definition(req),
            _ => Err(Error::BadRequest(
                method,
                "method cannot be request".to_owned(),
            )),
        }
    }

    fn handle_response(&self, resp: Response) -> Result<(), Error> {
        info!("got response: {resp:?}");
        Err(Error::UnsupportedMethod("Response".to_owned()))
    }

    fn handle_notification(&mut self, not: Notification) -> Result<(), Error> {
        let method = not.method.parse::<Method>()?;
        match method {
            Method::DidOpen => self.did_open(not),
            _ => Err(Error::BadRequest(
                method,
                "method cannot be notification".to_owned(),
            )),
        }
    }

    fn goto_definition(&mut self, req: Request) -> Result<Response, Error> {
        let (id, params) =
            req.extract::<GotoDefinitionParams>(&Method::GotoDefinition.to_string())?;

        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let ident = self.doc.get_ident(pos)?;
        let loc = self.doc.find_ident(ident, uri)?;

        let result = Some(GotoDefinitionResponse::Scalar(loc));
        let result = serde_json::to_value(&result)?;
        let resp = Response {
            id,
            result: Some(result),
            error: None,
        };
        Ok(resp)
    }

    fn did_open(&mut self, not: Notification) -> Result<(), Error> {
        let params = not.extract::<DidOpenTextDocumentParams>(&Method::DidOpen.to_string())?;
        let new_doc = Document::from_text(params.text_document.text)?;
        self.doc = new_doc;
        Ok(())
    }
}
