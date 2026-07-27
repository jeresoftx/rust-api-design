//! Solución del ejercicio del capítulo 9.

use rust_api_design::security_boundary::{DataSensitivity, ErrorExposure, SecurityBoundary};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boundary = SecurityBoundary::new(
        "importe",
        DataSensitivity::Sensitive,
        ErrorExposure::SafeMessage,
    )?;

    println!(
        "¿Entrada extensa rechazada? {}",
        boundary.rejects(&"x".repeat(129))
    );
    Ok(())
}
