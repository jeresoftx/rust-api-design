//! Solución del ejercicio 3 del capítulo 1.

use rust_api_design::contracts::{
    ActionableError, ApiContract, Presence, PrivateDetail, PublicField,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payment_contract = ApiContract::new(
        "registrar un pago",
        vec![
            PublicField::new(
                "factura_id",
                "identificador estable de la factura que se desea pagar",
                Presence::Required,
            )?,
            PublicField::new(
                "importe",
                "cantidad que el consumidor solicita registrar como pago",
                Presence::Required,
            )?,
        ],
        vec![
            PublicField::new(
                "pago_id",
                "identificador estable del pago registrado",
                Presence::Required,
            )?,
            PublicField::new(
                "estado",
                "resultado actual del registro que el consumidor puede mostrar",
                Presence::Required,
            )?,
        ],
        vec![
            ActionableError::new(
                "factura_no_encontrada",
                "corregir el identificador de factura antes de reintentar",
            )?,
            ActionableError::new(
                "factura_ya_pagada",
                "consultar el estado actual y detener un nuevo registro",
            )?,
        ],
        vec![PrivateDetail::new(
            "la estrategia de persistencia y conciliación del pago",
        )?],
    )?;

    assert_eq!(payment_contract.inputs().len(), 2);
    assert_eq!(payment_contract.outputs().len(), 2);
    assert_eq!(payment_contract.errors()[1].code(), "factura_ya_pagada");

    println!("Contrato creado: {}", payment_contract.capability());

    Ok(())
}
