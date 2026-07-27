//! Ejemplo ejecutable del capítulo 5: declarar resultados observables.

use rust_api_design::executable_contract::{DeclaredResponse, OperationSpec};
use rust_api_design::http::{HttpMethod, HttpStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operation = OperationSpec::new(
        "getPayment",
        HttpMethod::Get,
        "/payments/{payment_id}",
        vec![
            DeclaredResponse::new(HttpStatus::Ok, "Pago encontrado")?,
            DeclaredResponse::new(HttpStatus::NotFound, "Pago no encontrado")?,
        ],
    )?;

    println!(
        "{:?} {} declara {} respuestas.",
        operation.method(),
        operation.path(),
        operation.responses().len()
    );
    Ok(())
}
