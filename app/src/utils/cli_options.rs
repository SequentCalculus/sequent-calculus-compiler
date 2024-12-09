#[derive(clap::ValueEnum, Clone)]
pub enum Backend {
    Aarch64,
    Rv64,
    X86_64,
}
