//! Solución del ejercicio del capítulo 7.

use rust_api_design::operation_policy::{
    CachePolicy, OperationPolicy, OperationSafety, RateLimit, RetryRule,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let update_address = OperationPolicy::new(
        CachePolicy::NoStore,
        RateLimit::new(30, 60, 20)?,
        OperationSafety::NonIdempotent,
        RetryRule::Never,
    )?;

    println!("Reintento automático: {:?}.", update_address.retry());
    Ok(())
}
