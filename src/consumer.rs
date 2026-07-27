//! Modelos de errores y paginación para consumidores de una API.

use std::fmt;

/// Detalle seguro que localiza una corrección en una solicitud.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationDetail {
    field: String,
    rule: String,
}

impl ValidationDetail {
    pub fn new(field: impl Into<String>, rule: impl Into<String>) -> Result<Self, ConsumerError> {
        Ok(Self {
            field: required_text(field.into(), "campo")?,
            rule: required_text(rule.into(), "regla")?,
        })
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn rule(&self) -> &str {
        &self.rule
    }
}

/// Error público con una decisión estable para el consumidor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    code: String,
    message: String,
    details: Vec<ValidationDetail>,
}

impl ApiError {
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        details: Vec<ValidationDetail>,
    ) -> Result<Self, ConsumerError> {
        Ok(Self {
            code: required_text(code.into(), "código de error")?,
            message: required_text(message.into(), "mensaje de error")?,
            details,
        })
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn details(&self) -> &[ValidationDetail] {
        &self.details
    }
}

/// Orden que hace recorrible una colección paginada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableOrder {
    description: String,
}

impl StableOrder {
    pub fn new(description: impl Into<String>) -> Result<Self, ConsumerError> {
        Ok(Self {
            description: required_text(description.into(), "orden estable")?,
        })
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Continuación opaca de una consulta paginada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor(String);

impl Cursor {
    pub fn new(value: impl Into<String>) -> Result<Self, ConsumerError> {
        Ok(Self(required_text(value.into(), "cursor")?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Resultado paginado que conserva límite, orden y continuación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    items: Vec<T>,
    limit: usize,
    order: StableOrder,
    next_cursor: Option<Cursor>,
}

impl<T> Page<T> {
    pub fn new(
        items: Vec<T>,
        limit: usize,
        order: StableOrder,
        next_cursor: Option<Cursor>,
    ) -> Result<Self, ConsumerError> {
        if limit == 0 {
            return Err(ConsumerError::ZeroLimit);
        }

        if items.len() > limit {
            return Err(ConsumerError::ItemsExceedLimit {
                items: items.len(),
                limit,
            });
        }

        Ok(Self {
            items,
            limit,
            order,
            next_cursor,
        })
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn order(&self) -> &StableOrder {
        &self.order
    }

    pub fn next_cursor(&self) -> Option<&Cursor> {
        self.next_cursor.as_ref()
    }
}

/// Errores del modelo que protegen promesas para consumidores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsumerError {
    EmptyText { subject: &'static str },
    ZeroLimit,
    ItemsExceedLimit { items: usize, limit: usize },
}

impl fmt::Display for ConsumerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(formatter, "El {subject} no puede estar vacío."),
            Self::ZeroLimit => write!(
                formatter,
                "El límite de una página debe ser mayor que cero."
            ),
            Self::ItemsExceedLimit { items, limit } => {
                write!(
                    formatter,
                    "La página tiene {items} elementos y su límite es {limit}."
                )
            }
        }
    }
}

impl std::error::Error for ConsumerError {}

fn required_text(value: String, subject: &'static str) -> Result<String, ConsumerError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(ConsumerError::EmptyText { subject });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{ApiError, ConsumerError, Cursor, Page, StableOrder, ValidationDetail};

    #[test]
    fn exposes_a_stable_error_with_field_details() {
        let error = ApiError::new(
            "importe_invalido",
            "El importe debe ser positivo.",
            vec![ValidationDetail::new("importe", "mayor que cero").unwrap()],
        )
        .unwrap();

        assert_eq!(error.code(), "importe_invalido");
        assert_eq!(error.details()[0].field(), "importe");
    }

    #[test]
    fn creates_a_page_with_order_and_opaque_continuation() {
        let page = Page::new(
            vec!["pago-1", "pago-2"],
            2,
            StableOrder::new("created_at asc, id asc").unwrap(),
            Some(Cursor::new("continuacion-opaca").unwrap()),
        )
        .unwrap();

        assert_eq!(page.order().description(), "created_at asc, id asc");
        assert_eq!(page.next_cursor().unwrap().as_str(), "continuacion-opaca");
    }

    #[test]
    fn rejects_more_items_than_the_declared_limit() {
        let error =
            Page::new(vec![1, 2], 1, StableOrder::new("id asc").unwrap(), None).unwrap_err();

        assert_eq!(
            error,
            ConsumerError::ItemsExceedLimit { items: 2, limit: 1 }
        );
    }
}
