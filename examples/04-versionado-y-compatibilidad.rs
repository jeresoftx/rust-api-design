//! Ejemplo ejecutable del capítulo 4: transición de un cambio incompatible.

use rust_api_design::evolution::{ContractChange, DeprecationPlan, Migration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plan = DeprecationPlan::new("estado", "payment_status", "2027-01-01")?;
    let migration = Migration::new(ContractChange::ChangeFieldMeaning, plan)?;

    println!("Reemplazo: {}", migration.deprecation().replacement());
    Ok(())
}
