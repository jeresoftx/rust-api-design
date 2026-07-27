//! Ejemplo ejecutable del capítulo 7: declarar una política operativa honesta.

use rust_api_design::operation_policy::{
    CachePolicy, OperationPolicy, OperationSafety, RateLimit, RetryRule,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = OperationPolicy::new(
        CachePolicy::Shared {
            max_age_seconds: 300,
        },
        RateLimit::new(120, 60, 10)?,
        OperationSafety::ReadOnly,
        RetryRule::OnTransientFailure,
    )?;

    println!(
        "El catálogo puede indicar reintento después de {} segundos.",
        catalog.rate_limit().retry_after_seconds()
    );
    Ok(())
}
