//! Solución del ejercicio del capítulo 6.

use rust_api_design::style_selection::{
    ApiStyle, ConsumerInteraction, ContractLimit, StyleSelection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reservation = StyleSelection::new(
        ApiStyle::Grpc,
        ConsumerInteraction::TypedServiceCall,
        ContractLimit::RequestBoundary,
    )?;

    println!(
        "La reserva usa {:?} con {:?}.",
        reservation.style(),
        reservation.limit()
    );
    Ok(())
}
