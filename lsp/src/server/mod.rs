use crate::errors::Error;
use log::warn;
use lsp_server::{Connection, IoThreads, Message};
use lsp_types::{InitializeParams, ServerCapabilities};
use lsp_types::{OneOf, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions};

pub mod document;
mod message_handler;
pub mod method;
use message_handler::MessageHandler;

pub struct LspServer {
    conn: Connection,
    threads: IoThreads,
    handler: MessageHandler,
}
impl LspServer {
    pub fn new() -> Result<LspServer, Error> {
        let (connection, io_threads) = Connection::stdio();
        let server_capabilities = serde_json::to_value(&Self::capabilities())?;
        let initialization_params = connection.initialize(server_capabilities)?;
        let _: InitializeParams = serde_json::from_value(initialization_params)?;
        Ok(LspServer {
            conn: connection,
            threads: io_threads,
            handler: MessageHandler::new(),
        })
    }

    fn capabilities() -> ServerCapabilities {
        ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::FULL),
                    ..Default::default()
                },
            )),
            definition_provider: Some(OneOf::Left(true)),
            ..Default::default()
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        for msg in self.conn.receiver.iter() {
            if let Message::Request(ref req) = msg {
                if self.conn.handle_shutdown(&req)? {
                    return Ok(());
                }
            }
            match self.handler.handle_message(msg) {
                Err(err) => {
                    warn!("Server encountered error: {err}");
                    Ok(())
                }
                Ok(None) => Ok(()),
                Ok(Some(resp)) => self.conn.sender.send(resp),
            }?;
        }
        self.threads.join()?;
        Ok(())
    }
}
