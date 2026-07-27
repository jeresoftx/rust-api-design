//! Ejemplo ejecutable del capítulo 9: validar sin filtrar detalles internos.

use rust_api_design::security_boundary::{DataSensitivity, ErrorExposure, SecurityBoundary};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boundary = SecurityBoundary::new(
        "importe",
        DataSensitivity::Sensitive,
        ErrorExposure::SafeMessage,
    )?;

    println!("¿La entrada vacía se rechaza? {}", boundary.rejects(""));
    Ok(())
}
