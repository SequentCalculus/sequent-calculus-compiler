use crate::errors::Error;
use log::info;
use lsp_server::{Connection, IoThreads, Message};
use lsp_types::OneOf;
use lsp_types::{InitializeParams, ServerCapabilities};

mod message_handler;
mod method;
use message_handler::MessageHandler;

pub struct LspServer {
    conn: Connection,
    threads: IoThreads,
    handler: MessageHandler,
}
impl LspServer {
    pub fn new() -> Result<LspServer, Error> {
        let (connection, io_threads) = Connection::stdio();
        let server_capabilities = serde_json::to_value(&ServerCapabilities {
            definition_provider: Some(OneOf::Left(true)),
            ..Default::default()
        })?;
        let initialization_params = connection.initialize(server_capabilities)?;
        let _: InitializeParams = serde_json::from_value(initialization_params)?;
        Ok(LspServer {
            conn: connection,
            threads: io_threads,
            handler: MessageHandler::new(),
        })
    }

    pub fn run(mut self) -> Result<(), Error> {
        info!("starting example main loop");
        let msg_iter = self.conn.receiver.iter();
        for msg in msg_iter {
            if let Message::Request(ref req) = msg {
                if self.conn.handle_shutdown(&req)? {
                    return Ok(());
                }
            }
            let resp = self.handler.handle_message(msg)?;
            self.conn.sender.send(Message::Response(resp))?;
        }
        self.threads.join()?;
        Ok(())
    }
}
