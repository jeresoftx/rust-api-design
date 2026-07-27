//! Ejemplo ejecutable del capítulo 6: elegir un estilo y declarar su límite.

use rust_api_design::style_selection::{
    ApiStyle, ConsumerInteraction, ContractLimit, StyleSelection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = StyleSelection::new(
        ApiStyle::GraphQl,
        ConsumerInteraction::ComposedView,
        ContractLimit::QueryBudget {
            max_depth: 4,
            max_fields: 40,
        },
    )?;
    let events = StyleSelection::new(
        ApiStyle::Grpc,
        ConsumerInteraction::Streaming,
        ContractLimit::StreamWindow { max_in_flight: 16 },
    )?;

    println!(
        "Catálogo: {:?}; eventos: {:?}.",
        catalog.limit(),
        events.limit()
    );
    Ok(())
}
