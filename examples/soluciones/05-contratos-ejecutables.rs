//! Solución del ejercicio del capítulo 5.

use rust_api_design::executable_contract::{DeclaredResponse, OperationSpec};
use rust_api_design::http::{HttpMethod, HttpStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operation = OperationSpec::new(
        "createPayment",
        HttpMethod::Post,
        "/payments",
        vec![
            DeclaredResponse::new(HttpStatus::Created, "Pago creado")?,
            DeclaredResponse::new(HttpStatus::UnprocessableContent, "Entrada inválida")?,
        ],
    )?;

    println!(
        "{} declara {} respuestas.",
        operation.operation_id(),
        operation.responses().len()
    );
    Ok(())
}
