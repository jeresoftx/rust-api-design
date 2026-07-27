//! Semántica HTTP mínima para el capítulo sobre REST.
//!
//! Este módulo modela decisiones de contrato, no un servidor. Su objetivo es
//! detectar combinaciones de método, resultado y reintento que un consumidor
//! no podría interpretar con confianza.

use std::fmt;

/// Intención protocolaria de una operación HTTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    /// Indica si observar la operación no debe cambiar el estado de negocio.
    pub fn is_safe(self) -> bool {
        matches!(self, Self::Get)
    }

    /// Indica si el protocolo define el método como idempotente.
    pub fn is_idempotent(self) -> bool {
        matches!(self, Self::Get | Self::Put | Self::Delete)
    }
}

/// Resultado HTTP relevante para una decisión del consumidor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    UnprocessableContent,
}

impl HttpStatus {
    /// Código numérico que viaja por la respuesta HTTP.
    pub fn code(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NoContent => 204,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::UnprocessableContent => 422,
        }
    }
}

/// Garantía que permite o prohíbe repetir una solicitud ante incertidumbre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryPolicy {
    /// El consumidor no debe repetir la solicitud automáticamente.
    Never,
    /// El método HTTP es suficiente para sostener un reintento equivalente.
    ByMethod,
    /// Una clave de idempotencia protege una operación que podría duplicarse.
    IdempotencyKey,
}

/// Forma de observar una operación aceptada que termina de forma asíncrona.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FollowUpResource {
    location: String,
}

impl FollowUpResource {
    /// Crea la ubicación pública donde el consumidor podrá consultar el resultado.
    pub fn new(location: impl Into<String>) -> Result<Self, HttpDesignError> {
        let location = required_text(location.into(), "ubicación de seguimiento")?;

        if !location.starts_with('/') {
            return Err(HttpDesignError::InvalidFollowUpLocation { location });
        }

        Ok(Self { location })
    }

    /// URI pública de la operación aceptada.
    pub fn location(&self) -> &str {
        &self.location
    }
}

/// Declaración de una interacción HTTP que el consumidor puede observar.
///
/// ```
/// use rust_api_design::http::{HttpInteraction, HttpMethod, HttpStatus, RetryPolicy};
///
/// let interaction = HttpInteraction::new(
///     HttpMethod::Post,
///     HttpStatus::Created,
///     RetryPolicy::IdempotencyKey,
///     None,
/// )?;
///
/// assert_eq!(interaction.status().code(), 201);
/// # Ok::<(), rust_api_design::http::HttpDesignError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpInteraction {
    method: HttpMethod,
    status: HttpStatus,
    retry_policy: RetryPolicy,
    follow_up: Option<FollowUpResource>,
}

impl HttpInteraction {
    /// Construye una interacción que conserva la semántica de HTTP.
    pub fn new(
        method: HttpMethod,
        status: HttpStatus,
        retry_policy: RetryPolicy,
        follow_up: Option<FollowUpResource>,
    ) -> Result<Self, HttpDesignError> {
        if method.is_safe() && changes_state(status) {
            return Err(HttpDesignError::SafeMethodChangesState { method, status });
        }

        if status == HttpStatus::Created && !matches!(method, HttpMethod::Post | HttpMethod::Put) {
            return Err(HttpDesignError::CreatedWithIncompatibleMethod { method });
        }

        if retry_policy == RetryPolicy::ByMethod && !method.is_idempotent() {
            return Err(HttpDesignError::MethodIsNotIdempotent { method });
        }

        if status == HttpStatus::Accepted && follow_up.is_none() {
            return Err(HttpDesignError::AcceptedWithoutFollowUp);
        }

        if status != HttpStatus::Accepted && follow_up.is_some() {
            return Err(HttpDesignError::FollowUpWithoutAccepted);
        }

        Ok(Self {
            method,
            status,
            retry_policy,
            follow_up,
        })
    }

    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn status(&self) -> HttpStatus {
        self.status
    }

    pub fn retry_policy(&self) -> RetryPolicy {
        self.retry_policy
    }

    pub fn follow_up(&self) -> Option<&FollowUpResource> {
        self.follow_up.as_ref()
    }
}

/// Errores que revelan un desacuerdo entre intención y semántica HTTP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpDesignError {
    EmptyText {
        subject: &'static str,
    },
    InvalidFollowUpLocation {
        location: String,
    },
    SafeMethodChangesState {
        method: HttpMethod,
        status: HttpStatus,
    },
    CreatedWithIncompatibleMethod {
        method: HttpMethod,
    },
    MethodIsNotIdempotent {
        method: HttpMethod,
    },
    AcceptedWithoutFollowUp,
    FollowUpWithoutAccepted,
}

impl fmt::Display for HttpDesignError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(formatter, "El {subject} no puede estar vacío."),
            Self::InvalidFollowUpLocation { location } => {
                write!(formatter, "La ubicación `{location}` debe iniciar con `/`.")
            }
            Self::SafeMethodChangesState { .. } => {
                write!(
                    formatter,
                    "Un método seguro no puede comunicar un cambio de estado."
                )
            }
            Self::CreatedWithIncompatibleMethod { .. } => {
                write!(
                    formatter,
                    "201 Created solo es coherente con POST o PUT en este modelo."
                )
            }
            Self::MethodIsNotIdempotent { .. } => {
                write!(
                    formatter,
                    "El método no permite declarar reintento por semántica propia."
                )
            }
            Self::AcceptedWithoutFollowUp => {
                write!(
                    formatter,
                    "202 Accepted requiere una ubicación pública de seguimiento."
                )
            }
            Self::FollowUpWithoutAccepted => {
                write!(
                    formatter,
                    "La ubicación de seguimiento solo corresponde a 202 Accepted."
                )
            }
        }
    }
}

impl std::error::Error for HttpDesignError {}

fn changes_state(status: HttpStatus) -> bool {
    matches!(
        status,
        HttpStatus::Created | HttpStatus::Accepted | HttpStatus::NoContent
    )
}

fn required_text(value: String, subject: &'static str) -> Result<String, HttpDesignError> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(HttpDesignError::EmptyText { subject });
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{
        FollowUpResource, HttpDesignError, HttpInteraction, HttpMethod, HttpStatus, RetryPolicy,
    };

    #[test]
    fn accepts_a_post_created_with_an_idempotency_key() {
        let interaction = HttpInteraction::new(
            HttpMethod::Post,
            HttpStatus::Created,
            RetryPolicy::IdempotencyKey,
            None,
        )
        .unwrap();

        assert_eq!(interaction.status().code(), 201);
        assert_eq!(interaction.retry_policy(), RetryPolicy::IdempotencyKey);
    }

    #[test]
    fn rejects_a_get_that_claims_to_create_a_resource() {
        let error = HttpInteraction::new(
            HttpMethod::Get,
            HttpStatus::Created,
            RetryPolicy::ByMethod,
            None,
        )
        .unwrap_err();

        assert_eq!(
            error,
            HttpDesignError::SafeMethodChangesState {
                method: HttpMethod::Get,
                status: HttpStatus::Created,
            }
        );
    }

    #[test]
    fn rejects_post_retry_by_method() {
        let error = HttpInteraction::new(
            HttpMethod::Post,
            HttpStatus::Ok,
            RetryPolicy::ByMethod,
            None,
        )
        .unwrap_err();

        assert_eq!(
            error,
            HttpDesignError::MethodIsNotIdempotent {
                method: HttpMethod::Post,
            }
        );
    }

    #[test]
    fn requires_a_follow_up_resource_for_asynchronous_work() {
        let error = HttpInteraction::new(
            HttpMethod::Post,
            HttpStatus::Accepted,
            RetryPolicy::Never,
            None,
        )
        .unwrap_err();

        assert_eq!(error, HttpDesignError::AcceptedWithoutFollowUp);
    }

    #[test]
    fn exposes_follow_up_for_an_accepted_operation() {
        let interaction = HttpInteraction::new(
            HttpMethod::Post,
            HttpStatus::Accepted,
            RetryPolicy::IdempotencyKey,
            Some(FollowUpResource::new("/operaciones/42").unwrap()),
        )
        .unwrap();

        assert_eq!(
            interaction.follow_up().unwrap().location(),
            "/operaciones/42"
        );
    }
}
