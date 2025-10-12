use spomo::error::{AppResult, AppError};
use spomo::init;
use error_stack::bail;
use tracing::{error, info, trace, warn};

fn main() -> AppResult<()> {
    init::error_reporting();
    init::tracing();

    println!("Hello");

    info!("Sample log message");
    warn!("Warning message");
    error!("Error message");
    trace!("Tracing message");

    //bail!(AppError);
    Ok(())
}