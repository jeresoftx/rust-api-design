//! Ejemplo ejecutable del capítulo 1: promesa pública y límite privado.

use rust_api_design::contracts::{
    ActionableError, ApiContract, Presence, PrivateDetail, PublicField,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract = ApiContract::new(
        "consultar un pedido",
        vec![PublicField::new(
            "pedido_id",
            "identificador estable del pedido",
            Presence::Required,
        )?],
        vec![PublicField::new(
            "estado",
            "estado actual que el consumidor puede mostrar",
            Presence::Required,
        )?],
        vec![ActionableError::new(
            "pedido_no_encontrado",
            "corregir el identificador o dejar de consultar",
        )?],
        vec![PrivateDetail::new(
            "la estrategia de búsqueda y almacenamiento del pedido",
        )?],
    )?;

    println!("Capacidad: {}", contract.capability());
    println!("Salida pública: {}", contract.outputs()[0].name());
    println!("Error accionable: {}", contract.errors()[0].code());

    Ok(())
}
