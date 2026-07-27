//! Solución del ejercicio del capítulo 10.

use rust_api_design::strategy_governance::{ApiCapability, ChangeImpact};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capability = ApiCapability::new("payments", "mobile", ChangeImpact::High)?;

    println!("Revisión humana: {}", capability.requires_human_review());
    Ok(())
}
