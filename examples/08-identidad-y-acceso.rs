//! Ejemplo ejecutable del capítulo 8: identidad y permiso son decisiones distintas.

use rust_api_design::identity_access::{AccessRequest, Action, Credential, decide};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Credential::new("ana", "payments-api")?;
    let request = AccessRequest::new(credential, Action::Update, "payment:123")?;
    let decision = decide(&request, "payments-api", "ana", Action::Read)?;

    println!("Decisión: {:?}.", decision);
    Ok(())
}
