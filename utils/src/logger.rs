use slog::Drain;
use slog::o;

use super::error::Result;

pub fn setup_logging() -> Result<()> {
    // Setup Logging
    let _guard = slog_scope::set_global_logger(default_root_logger()?);
    slog_stdlog::init().unwrap();

    Ok(())
}

pub fn default_root_logger() -> Result<slog::Logger> {
    // Create drains - temporarily disable syslog due to security vulnerability
    let term_drain = default_term_drain().unwrap_or(default_discard()?);

    // Use only terminal drain for now
    let drain = term_drain.fuse();

    // Create Logger
    let logger = slog::Logger::root(drain, o!("who" => "rust-starter"));

    // Return Logger
    Ok(logger)
}

fn default_discard() -> Result<slog_async::Async> {
    let drain = slog_async::Async::default(slog::Discard);

    Ok(drain)
}

// term drain: Log to Terminal
fn default_term_drain() -> Result<slog_async::Async> {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let term = slog_term::FullFormat::new(plain);

    let drain = slog_async::Async::default(term.build().fuse());

    Ok(drain)
}

