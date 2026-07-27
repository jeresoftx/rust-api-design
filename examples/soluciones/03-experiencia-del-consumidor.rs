//! Solución del ejercicio del capítulo 3.

use rust_api_design::consumer::{ApiError, Cursor, Page, StableOrder, ValidationDetail};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let error = ApiError::new(
        "importe_invalido",
        "El importe debe ser mayor que cero.",
        vec![ValidationDetail::new("importe", "mayor que cero")?],
    )?;
    let page = Page::new(
        vec!["pago-10", "pago-11"],
        2,
        StableOrder::new("created_at asc, id asc")?,
        Some(Cursor::new("continuacion-opaca-2")?),
    )?;

    println!("Corrige: {}", error.details()[0].field());
    println!("Siguiente cursor: {}", page.next_cursor().unwrap().as_str());
    Ok(())
}
