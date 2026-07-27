//! Solución del ejercicio 3 del capítulo 2.

use rust_api_design::http::{
    FollowUpResource, HttpInteraction, HttpMethod, HttpStatus, RetryPolicy,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let export = HttpInteraction::new(
        HttpMethod::Post,
        HttpStatus::Accepted,
        RetryPolicy::IdempotencyKey,
        Some(FollowUpResource::new("/operaciones/exportacion-99")?),
    )?;

    assert_eq!(export.status().code(), 202);
    assert_eq!(
        export.follow_up().unwrap().location(),
        "/operaciones/exportacion-99"
    );

    println!(
        "Consulta el resultado en: {}",
        export.follow_up().unwrap().location()
    );
    Ok(())
}
