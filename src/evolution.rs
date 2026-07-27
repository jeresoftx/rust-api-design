//! Vocabulario mínimo para razonar sobre compatibilidad de contratos.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractChange {
    AddOptionalField,
    AddOperation,
    RemoveField,
    MakeInputRequired,
    ChangeFieldMeaning,
    ChangePaginationOrder,
    ReuseErrorCode,
}

impl ContractChange {
    pub fn compatibility(self) -> Compatibility {
        match self {
            Self::AddOptionalField | Self::AddOperation => Compatibility::Compatible,
            Self::RemoveField
            | Self::MakeInputRequired
            | Self::ChangeFieldMeaning
            | Self::ChangePaginationOrder
            | Self::ReuseErrorCode => Compatibility::RequiresMigration,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compatibility {
    Compatible,
    RequiresMigration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeprecationPlan {
    replaced_behavior: String,
    replacement: String,
    sunset_date: String,
}

impl DeprecationPlan {
    pub fn new(
        replaced_behavior: impl Into<String>,
        replacement: impl Into<String>,
        sunset_date: impl Into<String>,
    ) -> Result<Self, EvolutionError> {
        Ok(Self {
            replaced_behavior: required_text(replaced_behavior.into(), "comportamiento deprecado")?,
            replacement: required_text(replacement.into(), "reemplazo")?,
            sunset_date: required_text(sunset_date.into(), "fecha de retirada")?,
        })
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Migration {
    change: ContractChange,
    deprecation: DeprecationPlan,
}

impl Migration {
    pub fn new(
        change: ContractChange,
        deprecation: DeprecationPlan,
    ) -> Result<Self, EvolutionError> {
        if change.compatibility() == Compatibility::Compatible {
            return Err(EvolutionError::CompatibleChangeDoesNotNeedMigration { change });
        }
        Ok(Self {
            change,
            deprecation,
        })
    }

    pub fn deprecation(&self) -> &DeprecationPlan {
        &self.deprecation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvolutionError {
    EmptyText { subject: &'static str },
    CompatibleChangeDoesNotNeedMigration { change: ContractChange },
}

impl fmt::Display for EvolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(formatter, "El {subject} no puede estar vacío."),
            Self::CompatibleChangeDoesNotNeedMigration { .. } => {
                write!(
                    formatter,
                    "Un cambio compatible no requiere una migración obligatoria."
                )
            }
        }
    }
}

impl std::error::Error for EvolutionError {}

fn required_text(value: String, subject: &'static str) -> Result<String, EvolutionError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(EvolutionError::EmptyText { subject });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{Compatibility, ContractChange, DeprecationPlan, EvolutionError, Migration};

    #[test]
    fn classifies_optional_addition_as_compatible() {
        assert_eq!(
            ContractChange::AddOptionalField.compatibility(),
            Compatibility::Compatible
        );
    }

    #[test]
    fn requires_a_plan_for_a_meaning_change() {
        let plan = DeprecationPlan::new("estado", "payment_status", "2027-01-01").unwrap();
        let migration = Migration::new(ContractChange::ChangeFieldMeaning, plan).unwrap();
        assert_eq!(migration.deprecation().replacement(), "payment_status");
    }

    #[test]
    fn rejects_a_migration_for_a_compatible_change() {
        let plan = DeprecationPlan::new("campo", "campo_nuevo", "2027-01-01").unwrap();
        let error = Migration::new(ContractChange::AddOperation, plan).unwrap_err();
        assert_eq!(
            error,
            EvolutionError::CompatibleChangeDoesNotNeedMigration {
                change: ContractChange::AddOperation
            }
        );
    }
}
