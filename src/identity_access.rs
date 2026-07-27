//! Modelo mínimo para distinguir identidad y decisión de acceso.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credential {
    subject: String,
    audience: String,
}

impl Credential {
    pub fn new(
        subject: impl Into<String>,
        audience: impl Into<String>,
    ) -> Result<Self, AccessError> {
        Ok(Self {
            subject: required_text(subject.into(), "sujeto")?,
            audience: required_text(audience.into(), "audiencia")?,
        })
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Read,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessRequest {
    credential: Credential,
    action: Action,
    resource: String,
}

impl AccessRequest {
    pub fn new(
        credential: Credential,
        action: Action,
        resource: impl Into<String>,
    ) -> Result<Self, AccessError> {
        Ok(Self {
            credential,
            action,
            resource: required_text(resource.into(), "recurso")?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessDecision {
    Allowed,
    Denied,
}

pub fn decide(
    request: &AccessRequest,
    expected_audience: &str,
    allowed_subject: &str,
    allowed_action: Action,
) -> Result<AccessDecision, AccessError> {
    let expected_audience = required_text(expected_audience.to_owned(), "audiencia esperada")?;
    let allowed_subject = required_text(allowed_subject.to_owned(), "sujeto autorizado")?;

    if request.credential.audience != expected_audience {
        return Err(AccessError::AudienceMismatch);
    }

    Ok(
        if request.credential.subject == allowed_subject && request.action == allowed_action {
            AccessDecision::Allowed
        } else {
            AccessDecision::Denied
        },
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessError {
    EmptyText { subject: &'static str },
    AudienceMismatch,
}

impl fmt::Display for AccessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(formatter, "El {subject} no puede estar vacío."),
            Self::AudienceMismatch => {
                write!(formatter, "La credencial no corresponde a esta audiencia.")
            }
        }
    }
}

impl std::error::Error for AccessError {}

fn required_text(value: String, subject: &'static str) -> Result<String, AccessError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(AccessError::EmptyText { subject });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{AccessDecision, AccessError, AccessRequest, Action, Credential, decide};

    #[test]
    fn allows_the_authorized_subject_for_the_requested_action() {
        let credential = Credential::new("ana", "payments-api").unwrap();
        let request = AccessRequest::new(credential, Action::Read, "payment:123").unwrap();

        assert_eq!(
            decide(&request, "payments-api", "ana", Action::Read).unwrap(),
            AccessDecision::Allowed
        );
    }

    #[test]
    fn denies_an_authenticated_subject_without_the_required_capability() {
        let credential = Credential::new("ana", "payments-api").unwrap();
        let request = AccessRequest::new(credential, Action::Update, "payment:123").unwrap();

        assert_eq!(
            decide(&request, "payments-api", "ana", Action::Read).unwrap(),
            AccessDecision::Denied
        );
    }

    #[test]
    fn rejects_a_credential_for_another_audience() {
        let credential = Credential::new("ana", "orders-api").unwrap();
        let request = AccessRequest::new(credential, Action::Read, "payment:123").unwrap();

        assert_eq!(
            decide(&request, "payments-api", "ana", Action::Read).unwrap_err(),
            AccessError::AudienceMismatch
        );
    }
}
