use super::{document::Document, method::Method};
use crate::errors::Error;
use log::info;
use lsp_server::{Message, Notification, Request, RequestId, Response};
//use lsp_types::request::GotoImplementationResponse;
use lsp_types::request::{
    GotoImplementationParams,
    GotoImplementationResponse,
    GotoDeclarationParams,
    GotoDeclarationResponse,
};
use lsp_types::{
    DidChangeTextDocumentParams,
    DidOpenTextDocumentParams,
    GotoDefinitionParams,
    GotoDefinitionResponse,
    Hover,
    HoverContents,
    PublishDiagnosticsParams,
    Uri,
    
    Documentation,
    MarkupContent,
    MarkupKind,
    ParameterInformation,
    ParameterLabel,
    SignatureHelp,
    SignatureInformation,
};
use printer::Print;
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
            Method::GotoImplementation => {
                let (id, params) = match req
                    .extract::<GotoImplementationParams>(&Method::GotoImplementation.to_string())
                {
                    Ok(res) => res,
                    Err(err) => return Error::ExtractReq(err).to_response(req_id),
                };
                self.goto_implementation(id, params)
            }

            Method::GotoDeclaration => {
                let (id, params) = match req
                    .extract::<GotoDeclarationParams>(&Method::GotoDeclaration.to_string())
                {
                    Ok(res) => res,
                    Err(err) => return Error::ExtractReq(err).to_response(req_id),
                };
                self.goto_declaration(id, params)
            }

            Method::Formatting => {
                let (id, params) = match req
                    .extract::<lsp_types::DocumentFormattingParams>(&Method::Formatting.to_string())
                {
                    Ok(res) => res,
                    Err(err) => return Error::ExtractReq(err).to_response(req_id),
                };
                self.formatting(id, params)
            }
            Method::RangeFormatting => {
                let (id, params) = match req
                    .extract::<lsp_types::DocumentRangeFormattingParams>(&Method::RangeFormatting.to_string())
                {
                    Ok(res) => res,
                    Err(err) => return Error::ExtractReq(err).to_response(req_id),
                };
                self.range_formatting(id, params)
            }
            Method::Hover => {
                let (id, params) = match req
                    .extract::<lsp_types::HoverParams>(&Method::Hover.to_string())
                {
                    Ok(res) => res,
                    Err(err) => return Error::ExtractReq(err).to_response(req_id),
                };
                self.hover(id, params)
            }
            Method::SignatureHelp => {
                let (id, params) = match req
                    .extract::<lsp_types::SignatureHelpParams>(&Method::SignatureHelp.to_string())
                {
                    Ok(res) => res,
                    Err(err) => return Error::ExtractReq(err).to_response(req_id),
                };
                self.signature_help(id, params)
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

    //eigener Code
    fn goto_implementation(&mut self, id: RequestId, params: GotoImplementationParams) -> Response {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let ident = match self.doc.get_ident(pos) {
            Ok(id) => id,
            Err(err) => return err.to_response(id),
        };

        let loc = match self.doc.find_implementation(ident, uri) {
            Ok(loc) => loc,
            Err(err) => return err.to_response(id),
        };

        let result = Some(GotoImplementationResponse::Scalar(loc));
        let result = serde_json::to_value(&result).unwrap();
        Response {
            id,
            result: Some(result),
            error: None,
        }
    }
    
    //eigener Code
    fn goto_declaration(&mut self, id: RequestId, params: GotoDeclarationParams) -> Response {

        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let ident = match self.doc.get_ident(pos) {
            Ok(id) => id,
            Err(err) => return err.to_response(id),
        };

        let loc = match self.doc.find_declaration(ident, uri) {
            Ok(loc) => loc,
            Err(err) => return err.to_response(id),
        };

        let result = Some(GotoDeclarationResponse::Scalar(loc));
        let result = serde_json::to_value(&result).unwrap();
        Response {
            id,
            result: Some(result),
            error: None,
        }
    }

    
    //eigener Code
    fn formatting(&mut self, id: RequestId, params: lsp_types::DocumentFormattingParams, ) -> Response {
        let opts = params.options;

        let mut formatted = self.doc.get_text().to_string();

        if opts.insert_spaces {
            let to_spaces = opts.tab_size as usize;
            formatted = formatted.lines().map(|line| line.replace("\t", &" ".repeat(to_spaces))).collect::<Vec<_>>().join("\n");

        } else{
             let to_tabs = opts.tab_size as usize;
            formatted = formatted.lines().map(|line| line.replace("\t", &" ".repeat(to_tabs))).collect::<Vec<_>>().join("\n");

        }
        if opts.trim_trailing_whitespace.unwrap_or(false) {
            formatted = formatted
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n");
        }

        if opts.insert_final_newline.unwrap_or(false) && !formatted.ends_with('\n') {
            formatted.push('\n');
        }

        if opts.trim_final_newlines.unwrap_or(false) {
            while formatted.ends_with("\n\n") {
                formatted.pop();
            }
        }

        let edit = lsp_types::TextEdit {
            range: lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: u32::MAX,
                    character: u32::MAX,
                },
            },
            new_text: formatted,
        };

        let text_edit = vec![edit];
        let result = serde_json::to_value(text_edit).unwrap();

        Response {
            id,
            result: Some(result),
            error: None,
        }
    }

    //eigener Code
    fn range_formatting(&mut self, id: RequestId, params: lsp_types::DocumentRangeFormattingParams, ) -> Response {
        let opts = params.options;
        let range = params.range;

        //extracting text 
        let  text_all = self.doc.get_text().to_string();
        let lines: Vec<&str> = text_all.lines().collect();

        //extracting lines and area of the selected text
        let line_start = range.start.line as usize;
        let line_end = range.end.line as usize;
        let area_start = range.start.character as usize;
        let area_end = range.end.character as usize;

        // lines with selected area in it
        let selected_lines: String = lines[line_start..=line_end].join("\n");
        //cutting lines with selected area in it to the exact area
        let mut selected_text: Vec<&str> = selected_lines.lines().collect();

        if let Some(first) = selected_text.first_mut(){
            if area_start > 0 {
                *first = &first[area_start..]
            }
        }

        if let Some(last) = selected_text.last_mut(){
            if area_end < last.len(){
                *last = &last[..area_end]
            }
        }

        let mut selected_text_area = selected_text.join("\n");

        if opts.insert_spaces {
            let to_spaces = opts.tab_size as usize;
            selected_text_area = selected_text_area.lines().map(|line| line.replace("\t", &" ".repeat(to_spaces))).collect::<Vec<_>>().join("\n");

        } else{
            let to_tabs = opts.tab_size as usize;
            selected_text_area = selected_text_area.lines().map(|line| line.replace("\t", &" ".repeat(to_tabs))).collect::<Vec<_>>().join("\n");

        }
        if opts.trim_trailing_whitespace.unwrap_or(false) {
            selected_text_area = selected_text_area.lines().map(|line| line.trim_end()).collect::<Vec<_>>().join("\n");
        }

        if opts.insert_final_newline.unwrap_or(false) && !selected_text_area.ends_with('\n') {
            selected_text_area.push('\n');
        }

        if opts.trim_final_newlines.unwrap_or(false) {
            while selected_text_area.ends_with("\n\n") {
                selected_text_area.pop();
            }
        }

        let edit = lsp_types::TextEdit {range, new_text:selected_text_area};

        let result = serde_json::to_value(vec![edit]).unwrap();

        Response {
            id,
            result: Some(result),
            error: None,
        }
    }

    //eigener Code
    fn hover(&mut self, id: RequestId, params: lsp_types::HoverParams) -> Response {
        //let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let ident = match self.doc.get_ident(pos) {
            Ok(id) => id,
            Err(err) => return err.to_response(id),
        };

        //TODO testen eventuell noch ne range abfrage
        let range = match self.doc.get_rangeident(pos) {
            Ok(full_range) => Some(full_range),
            Err(_) => None,           
        };

        let hover_information = match self.doc.get_hover_information(&ident) {
            Ok(info) => info, 
            Err(err) => return err.to_response(id),
        };

        let hover_content = Hover {
            contents: HoverContents::Markup(lsp_types::MarkupContent { kind: lsp_types::MarkupKind::Markdown, value: hover_information }),
            range,
        };

        let result  = serde_json::to_value(&hover_content).unwrap();

        Response {
            id,
            result: Some(result),
            error: None,
        } 
    }

    fn signature_help(&mut self, id: RequestId, params: lsp_types::SignatureHelpParams) -> Response {
        let pos = params.text_document_position_params.position;

        let ident = match self.doc.get_ident(pos) {
            Ok(id) => id,
            Err(err) => return err.to_response(id),
        };

        let signature_information = match self.doc.get_signature_information(&ident) {
            Ok(info) => info, 
            Err(err) => return err.to_response(id),
        };

        let parameter: Vec<ParameterInformation> = if let Some(def) =  
            self.doc.module().defs.iter().find(|d| d.name == ident) {
            if def.context.bindings.is_empty(){
                vec![ParameterInformation{
                    label: ParameterLabel::Simple("no parameter". to_string()),
                    documentation: None,
                }]
            } else {
                def.context.bindings.iter().map(|conbin| ParameterInformation{
                label:ParameterLabel::Simple(format!("{}: {}", conbin.var, conbin.ty.print_to_string(None))),
                documentation: None,
                }).collect()
            }
        } else {
            vec![ParameterInformation{
            label: ParameterLabel::Simple("unknown function". to_string()),
            documentation: None,
        }]
        };
        let signature_info = SignatureInformation{
            label:ident.clone(),
            documentation:Some(Documentation::MarkupContent(MarkupContent { kind: MarkupKind::Markdown, value: signature_information })),
            parameters: Some(parameter),
            active_parameter:Some(0),
        };
        let signature_help = SignatureHelp{
            signatures: vec![signature_info],
            active_signature: Some(0),
            active_parameter: Some(0),
        };
        
        let result  = serde_json::to_value(&signature_help).unwrap();

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
