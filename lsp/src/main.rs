use log::{info, LevelFilter};
use std::path::PathBuf;

mod errors;
mod logger;
mod server;
use errors::Error;
use logger::Logger;
use server::LspServer;

fn main() -> Result<(), Error> {
    setup_log()?;
    let server = LspServer::new()?;
    info!("starting LSP server");
    server.run()?;

    info!("shutting down server");
    Ok(())
}

fn setup_log() -> Result<(), Error> {
    let logger = Logger::new(PathBuf::from("lsp.log"))?;
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}
