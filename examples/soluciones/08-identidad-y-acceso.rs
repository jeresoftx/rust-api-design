//! Solución del ejercicio del capítulo 8.

use rust_api_design::identity_access::{AccessRequest, Action, Credential, decide};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Credential::new("ana", "payments-api")?;
    let request = AccessRequest::new(credential, Action::Read, "payment:123")?;
    let decision = decide(&request, "payments-api", "ana", Action::Read)?;

    println!("Decisión: {:?}.", decision);
    Ok(())
}
