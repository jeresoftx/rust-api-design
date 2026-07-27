//! Modelo mínimo de una operación de contrato ejecutable.

use std::fmt;

use crate::http::{HttpMethod, HttpStatus};

/// Respuesta declarada para una operación de API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredResponse {
    status: HttpStatus,
    description: String,
}

impl DeclaredResponse {
    pub fn new(
        status: HttpStatus,
        description: impl Into<String>,
    ) -> Result<Self, ContractSpecError> {
        Ok(Self {
            status,
            description: required_text(description.into(), "descripción de respuesta")?,
        })
    }

    pub fn status(&self) -> HttpStatus {
        self.status
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Operación que puede revisarse y verificarse contra una implementación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationSpec {
    operation_id: String,
    method: HttpMethod,
    path: String,
    responses: Vec<DeclaredResponse>,
}

impl OperationSpec {
    pub fn new(
        operation_id: impl Into<String>,
        method: HttpMethod,
        path: impl Into<String>,
        responses: Vec<DeclaredResponse>,
    ) -> Result<Self, ContractSpecError> {
        let operation_id = required_text(operation_id.into(), "identificador de operación")?;
        let path = required_text(path.into(), "ruta")?;

        if !path.starts_with('/') {
            return Err(ContractSpecError::InvalidPath { path });
        }
        if responses.is_empty() {
            return Err(ContractSpecError::MissingResponses);
        }

        let mut statuses = std::collections::BTreeSet::new();
        for response in &responses {
            if !statuses.insert(response.status().code()) {
                return Err(ContractSpecError::DuplicateResponseStatus {
                    status: response.status(),
                });
            }
        }

        Ok(Self {
            operation_id,
            method,
            path,
            responses,
        })
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn responses(&self) -> &[DeclaredResponse] {
        &self.responses
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractSpecError {
    EmptyText { subject: &'static str },
    InvalidPath { path: String },
    MissingResponses,
    DuplicateResponseStatus { status: HttpStatus },
}

impl fmt::Display for ContractSpecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(formatter, "El {subject} no puede estar vacío."),
            Self::InvalidPath { path } => {
                write!(formatter, "La ruta `{path}` debe iniciar con `/`.")
            }
            Self::MissingResponses => write!(
                formatter,
                "Una operación debe declarar al menos una respuesta."
            ),
            Self::DuplicateResponseStatus { status } => {
                write!(
                    formatter,
                    "El estado {} está declarado más de una vez.",
                    status.code()
                )
            }
        }
    }
}

impl std::error::Error for ContractSpecError {}

fn required_text(value: String, subject: &'static str) -> Result<String, ContractSpecError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(ContractSpecError::EmptyText { subject });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{ContractSpecError, DeclaredResponse, OperationSpec};
    use crate::http::{HttpMethod, HttpStatus};

    #[test]
    fn declares_an_operation_with_success_and_error_responses() {
        let operation = OperationSpec::new(
            "getPayment",
            HttpMethod::Get,
            "/payments/{payment_id}",
            vec![
                DeclaredResponse::new(HttpStatus::Ok, "Pago encontrado").unwrap(),
                DeclaredResponse::new(HttpStatus::NotFound, "Pago no encontrado").unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(operation.responses().len(), 2);
        assert_eq!(operation.path(), "/payments/{payment_id}");
    }

    #[test]
    fn rejects_an_operation_without_responses() {
        let error = OperationSpec::new(
            "getPayment",
            HttpMethod::Get,
            "/payments/{payment_id}",
            vec![],
        )
        .unwrap_err();
        assert_eq!(error, ContractSpecError::MissingResponses);
    }

    #[test]
    fn rejects_duplicate_response_statuses() {
        let response = DeclaredResponse::new(HttpStatus::Ok, "Pago encontrado").unwrap();
        let error = OperationSpec::new(
            "getPayment",
            HttpMethod::Get,
            "/payments/{payment_id}",
            vec![response.clone(), response],
        )
        .unwrap_err();

        assert_eq!(
            error,
            ContractSpecError::DuplicateResponseStatus {
                status: HttpStatus::Ok
            }
        );
    }
}
