//! Modelo mínimo de una frontera de seguridad observable en una API.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSensitivity {
    Public,
    Sensitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorExposure {
    SafeMessage,
    InternalDetail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityBoundary {
    input_name: String,
    sensitivity: DataSensitivity,
    error_exposure: ErrorExposure,
}

impl SecurityBoundary {
    pub fn new(
        input_name: impl Into<String>,
        sensitivity: DataSensitivity,
        error_exposure: ErrorExposure,
    ) -> Result<Self, SecurityError> {
        let input_name = required_text(input_name.into(), "entrada")?;
        if sensitivity == DataSensitivity::Sensitive
            && error_exposure == ErrorExposure::InternalDetail
        {
            return Err(SecurityError::SensitiveDataCannotExposeInternalDetail);
        }
        Ok(Self {
            input_name,
            sensitivity,
            error_exposure,
        })
    }

    pub fn rejects(&self, value: &str) -> bool {
        value.trim().is_empty() || value.len() > 128
    }

    pub fn error_exposure(&self) -> ErrorExposure {
        self.error_exposure
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError {
    EmptyText { subject: &'static str },
    SensitiveDataCannotExposeInternalDetail,
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(f, "La {subject} no puede estar vacía."),
            Self::SensitiveDataCannotExposeInternalDetail => write!(
                f,
                "Los datos sensibles no pueden exponer detalles internos."
            ),
        }
    }
}
impl std::error::Error for SecurityError {}

fn required_text(value: String, subject: &'static str) -> Result<String, SecurityError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(SecurityError::EmptyText { subject });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{DataSensitivity, ErrorExposure, SecurityBoundary, SecurityError};

    #[test]
    fn accepts_a_safe_boundary_for_sensitive_input() {
        let boundary = SecurityBoundary::new(
            "importe",
            DataSensitivity::Sensitive,
            ErrorExposure::SafeMessage,
        )
        .unwrap();
        assert!(boundary.rejects(""));
    }

    #[test]
    fn rejects_internal_details_for_sensitive_input() {
        let error = SecurityBoundary::new(
            "importe",
            DataSensitivity::Sensitive,
            ErrorExposure::InternalDetail,
        )
        .unwrap_err();
        assert_eq!(
            error,
            SecurityError::SensitiveDataCannotExposeInternalDetail
        );
    }

    #[test]
    fn rejects_overlong_untrusted_input() {
        let boundary = SecurityBoundary::new(
            "consulta",
            DataSensitivity::Public,
            ErrorExposure::SafeMessage,
        )
        .unwrap();
        assert!(boundary.rejects(&"x".repeat(129)));
    }
}
