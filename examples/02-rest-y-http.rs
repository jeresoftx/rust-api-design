//! Ejemplo ejecutable del capítulo 2: creación idempotente y trabajo asíncrono.

use rust_api_design::http::{
    FollowUpResource, HttpInteraction, HttpMethod, HttpStatus, RetryPolicy,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payment = HttpInteraction::new(
        HttpMethod::Post,
        HttpStatus::Created,
        RetryPolicy::IdempotencyKey,
        None,
    )?;

    let export = HttpInteraction::new(
        HttpMethod::Post,
        HttpStatus::Accepted,
        RetryPolicy::IdempotencyKey,
        Some(FollowUpResource::new("/operaciones/exportacion-42")?),
    )?;

    println!("Pago: {}", payment.status().code());
    println!("Seguimiento: {}", export.follow_up().unwrap().location());

    Ok(())
}
