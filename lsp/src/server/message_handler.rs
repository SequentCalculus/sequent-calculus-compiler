use super::{document::Document, method::Method};
use crate::errors::Error;
use log::info;
use lsp_server::{Message, Notification, Request, RequestId, Response};
use lsp_types::{
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, GotoDefinitionParams,
    GotoDefinitionResponse, PublishDiagnosticsParams, Uri,
};

pub struct MessageHandler {
    doc: Document,
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {
            doc: Document::new(),
        }
    }

    fn clear_diagnostics(&self, uri: Uri) -> Notification {
        Notification {
            method: Method::PublishDiagnostics.to_string(),
            params: serde_json::to_value(PublishDiagnosticsParams {
                uri,
                diagnostics: vec![],
                version: None,
            })
            .unwrap(),
        }
    }

    pub fn handle_message(&mut self, msg: Message) -> Result<Option<Message>, Error> {
        match msg {
            Message::Request(req) => Ok(Some(Message::Response(self.handle_request(req)))),
            Message::Response(resp) => self.handle_response(resp).map(|_| None),
            Message::Notification(not) => self
                .handle_notification(not)
                .map(|not| Some(Message::Notification(not))),
        }
    }

    fn handle_request(&mut self, req: Request) -> Response {
        info!("got request with type {}", req.method);
        let req_id = req.id.clone();

        let method = match req.method.parse::<Method>() {
            Ok(met) => met,
            Err(err) => return err.to_response(req_id),
        };
        match method {
            Method::GotoDefinition => {
                let (id, params) = match req
                    .extract::<GotoDefinitionParams>(&Method::GotoDefinition.to_string())
                {
                    Ok(res) => res,
                    Err(err) => return Error::ExtractReq(err).to_response(req_id),
                };
                self.goto_definition(id, params)
            }
            _ => {
                Error::BadRequest(method, "method cannot be request".to_owned()).to_response(req_id)
            }
        }
    }

    fn handle_response(&self, resp: Response) -> Result<(), Error> {
        info!("got response with type: {resp:?}");
        Err(Error::UnsupportedMethod("Response".to_owned()))
    }

    fn handle_notification(&mut self, not: Notification) -> Result<Notification, Error> {
        info!("got notification with method {}", not.method);
        let method = not.method.parse::<Method>()?;
        match method {
            Method::DidOpen => {
                let params =
                    not.extract::<DidOpenTextDocumentParams>(&Method::DidOpen.to_string())?;
                Ok(self.did_open(params))
            }
            Method::DidChange => {
                let params =
                    not.extract::<DidChangeTextDocumentParams>(&Method::DidChange.to_string())?;
                Ok(self.did_change(params))
            }
            _ => Err(Error::BadRequest(
                method,
                "method cannot be notification".to_owned(),
            )),
        }
    }

    fn goto_definition(&mut self, id: RequestId, params: GotoDefinitionParams) -> Response {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let ident = match self.doc.get_ident(pos) {
            Ok(id) => id,
            Err(err) => return err.to_response(id),
        };

        let loc = match self.doc.find_ident(ident, uri) {
            Ok(loc) => loc,
            Err(err) => return err.to_response(id),
        };

        let result = Some(GotoDefinitionResponse::Scalar(loc));
        let result = serde_json::to_value(&result).unwrap();
        Response {
            id,
            result: Some(result),
            error: None,
        }
    }

    fn did_open(&mut self, params: DidOpenTextDocumentParams) -> Notification {
        match Document::from_text(params.text_document.text) {
            Ok(doc) => {
                self.doc = doc;
                self.clear_diagnostics(params.text_document.uri)
            }
            Err(err) => err.to_notification(params.text_document.uri, &self.doc),
        }
    }

    fn did_change(&mut self, params: DidChangeTextDocumentParams) -> Notification {
        match Document::from_text(params.content_changes[0].text.clone()) {
            Ok(doc) => {
                self.doc = doc;
                self.clear_diagnostics(params.text_document.uri)
            }
            Err(err) => err.to_notification(params.text_document.uri, &self.doc),
        }
    }
}
