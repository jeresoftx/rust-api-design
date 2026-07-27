//! Ejemplo ejecutable del capítulo 3: corrección y continuidad paginada.

use rust_api_design::consumer::{ApiError, Cursor, Page, StableOrder, ValidationDetail};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let error = ApiError::new(
        "importe_invalido",
        "El importe debe ser positivo.",
        vec![ValidationDetail::new("importe", "mayor que cero")?],
    )?;
    let page = Page::new(
        vec!["pago-1", "pago-2"],
        2,
        StableOrder::new("created_at asc, id asc")?,
        Some(Cursor::new("continuacion-opaca")?),
    )?;

    println!("Corrige: {}", error.details()[0].field());
    println!("Continúa con: {}", page.next_cursor().unwrap().as_str());
    Ok(())
}
