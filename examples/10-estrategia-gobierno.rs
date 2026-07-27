//! Ejemplo ejecutable del capítulo 10: asignar propiedad antes de evolucionar.

use rust_api_design::strategy_governance::{ApiCapability, ChangeImpact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payments = ApiCapability::new("payments", "mobile", ChangeImpact::High)?;

    println!(
        "¿Requiere revisión humana? {}",
        payments.requires_human_review()
    );
    Ok(())
}
