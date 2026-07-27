//! Modelo mínimo para propiedad y revisión de una capacidad pública.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeImpact {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiCapability {
    owner: String,
    consumer: String,
    impact: ChangeImpact,
}

impl ApiCapability {
    pub fn new(
        owner: impl Into<String>,
        consumer: impl Into<String>,
        impact: ChangeImpact,
    ) -> Result<Self, GovernanceError> {
        Ok(Self {
            owner: required(owner.into(), "dueño")?,
            consumer: required(consumer.into(), "consumidor")?,
            impact,
        })
    }

    pub fn requires_human_review(&self) -> bool {
        self.impact == ChangeImpact::High
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceError {
    EmptyText { subject: &'static str },
}

impl fmt::Display for GovernanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(f, "El {subject} no puede estar vacío."),
        }
    }
}
impl std::error::Error for GovernanceError {}

fn required(value: String, subject: &'static str) -> Result<String, GovernanceError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(GovernanceError::EmptyText { subject });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{ApiCapability, ChangeImpact, GovernanceError};
    #[test]
    fn requires_review_for_a_high_impact_capability() {
        let capability = ApiCapability::new("payments", "mobile", ChangeImpact::High).unwrap();
        assert!(capability.requires_human_review());
        assert_eq!(capability.owner(), "payments");
    }
    #[test]
    fn allows_low_impact_change_without_human_review() {
        let capability = ApiCapability::new("catalog", "web", ChangeImpact::Low).unwrap();
        assert!(!capability.requires_human_review());
    }
    #[test]
    fn rejects_capability_without_owner() {
        assert_eq!(
            ApiCapability::new("", "web", ChangeImpact::Low).unwrap_err(),
            GovernanceError::EmptyText { subject: "dueño" }
        );
    }
}
