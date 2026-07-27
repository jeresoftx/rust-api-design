//! Vocabulario mínimo para expresar un contrato público de API.
//!
//! El módulo no construye un servidor ni serializa datos. Su propósito es
//! volver discutible qué promete una operación y qué detalles permanecen
//! internos antes de elegir REST, GraphQL o gRPC.

use std::fmt;

/// Describe si un campo siempre debe estar presente para el consumidor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// El consumidor puede depender de que el campo esté presente.
    Required,
    /// El contrato permite omitir el campo de manera explícita.
    Optional,
}

/// Un dato que cruza la frontera pública de la API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicField {
    name: String,
    meaning: String,
    presence: Presence,
}

impl PublicField {
    /// Crea un campo público con un nombre y significado verificables.
    pub fn new(
        name: impl Into<String>,
        meaning: impl Into<String>,
        presence: Presence,
    ) -> Result<Self, ContractError> {
        let name = required_text(name.into(), "nombre del campo")?;
        let meaning = required_text(meaning.into(), "significado del campo")?;

        Ok(Self {
            name,
            meaning,
            presence,
        })
    }

    /// Nombre estable que el consumidor usa para identificar el dato.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Semántica que evita que el consumidor deduzca el dato desde ejemplos.
    pub fn meaning(&self) -> &str {
        &self.meaning
    }

    /// Regla de presencia observable para el consumidor.
    pub fn presence(&self) -> Presence {
        self.presence
    }
}

/// Un error que el consumidor puede distinguir y usar para tomar una acción.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionableError {
    code: String,
    consumer_action: String,
}

impl ActionableError {
    /// Crea una declaración de error con una acción concreta para el consumidor.
    pub fn new(
        code: impl Into<String>,
        consumer_action: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let code = required_text(code.into(), "código de error")?;
        let consumer_action = required_text(consumer_action.into(), "acción del consumidor")?;

        Ok(Self {
            code,
            consumer_action,
        })
    }

    /// Código público y estable del error.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Acción que el consumidor puede tomar después de reconocer el error.
    pub fn consumer_action(&self) -> &str {
        &self.consumer_action
    }
}

/// Detalle deliberadamente excluido de la promesa pública.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateDetail {
    description: String,
}

impl PrivateDetail {
    /// Declara una decisión que el proveedor puede cambiar sin romper el contrato.
    pub fn new(description: impl Into<String>) -> Result<Self, ContractError> {
        Ok(Self {
            description: required_text(description.into(), "detalle privado")?,
        })
    }

    /// Explicación del límite que no pertenece a la API pública.
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Promesa observable de una operación de API.
///
/// ```
/// use rust_api_design::contracts::{
///     ActionableError, ApiContract, Presence, PrivateDetail, PublicField,
/// };
///
/// let contract = ApiContract::new(
///     "confirmar un pedido",
///     vec![PublicField::new("pedido_id", "identificador del pedido", Presence::Required)?],
///     vec![PublicField::new("estado", "estado confirmado del pedido", Presence::Required)?],
///     vec![ActionableError::new("pedido_no_encontrado", "corregir el identificador")?],
///     vec![PrivateDetail::new("la tabla donde se almacena el pedido")?],
/// )?;
///
/// assert_eq!(contract.capability(), "confirmar un pedido");
/// assert_eq!(contract.outputs().len(), 1);
/// # Ok::<(), rust_api_design::contracts::ContractError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiContract {
    capability: String,
    inputs: Vec<PublicField>,
    outputs: Vec<PublicField>,
    errors: Vec<ActionableError>,
    private_details: Vec<PrivateDetail>,
}

impl ApiContract {
    /// Construye un contrato que tiene una capacidad, resultado y límites claros.
    pub fn new(
        capability: impl Into<String>,
        inputs: Vec<PublicField>,
        outputs: Vec<PublicField>,
        errors: Vec<ActionableError>,
        private_details: Vec<PrivateDetail>,
    ) -> Result<Self, ContractError> {
        let capability = required_text(capability.into(), "capacidad")?;

        if outputs.is_empty() {
            return Err(ContractError::MissingOutput);
        }

        ensure_unique_field_names(&inputs, &outputs)?;

        Ok(Self {
            capability,
            inputs,
            outputs,
            errors,
            private_details,
        })
    }

    /// Tarea que el consumidor puede solicitar, sin revelar almacenamiento.
    pub fn capability(&self) -> &str {
        &self.capability
    }

    /// Datos públicos que la operación acepta.
    pub fn inputs(&self) -> &[PublicField] {
        &self.inputs
    }

    /// Datos públicos que la operación promete devolver.
    pub fn outputs(&self) -> &[PublicField] {
        &self.outputs
    }

    /// Errores que el consumidor puede distinguir de forma accionable.
    pub fn errors(&self) -> &[ActionableError] {
        &self.errors
    }

    /// Decisiones del proveedor que no forman parte de la promesa pública.
    pub fn private_details(&self) -> &[PrivateDetail] {
        &self.private_details
    }
}

/// Errores que impiden declarar una promesa pública suficientemente clara.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    /// Falta texto en una parte que un consumidor debe poder entender.
    EmptyText { subject: &'static str },
    /// Una operación sin salida no explica qué resultado promete.
    MissingOutput,
    /// Un mismo nombre público aparece en más de una parte de la frontera.
    DuplicateField { name: String },
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { subject } => write!(formatter, "El {subject} no puede estar vacío."),
            Self::MissingOutput => {
                write!(formatter, "Un contrato debe declarar al menos una salida.")
            }
            Self::DuplicateField { name } => {
                write!(
                    formatter,
                    "El campo público `{name}` está declarado más de una vez."
                )
            }
        }
    }
}

impl std::error::Error for ContractError {}

fn required_text(value: String, subject: &'static str) -> Result<String, ContractError> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(ContractError::EmptyText { subject });
    }

    Ok(value)
}

fn ensure_unique_field_names(
    inputs: &[PublicField],
    outputs: &[PublicField],
) -> Result<(), ContractError> {
    let mut names = std::collections::BTreeSet::new();

    for field in inputs.iter().chain(outputs) {
        if !names.insert(field.name()) {
            return Err(ContractError::DuplicateField {
                name: field.name().to_owned(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        ActionableError, ApiContract, ContractError, Presence, PrivateDetail, PublicField,
    };

    fn field(name: &str) -> PublicField {
        PublicField::new(name, "significado público", Presence::Required).unwrap()
    }

    #[test]
    fn builds_a_contract_with_observable_promise_and_private_limit() {
        let contract = ApiContract::new(
            "consultar un pedido",
            vec![field("pedido_id")],
            vec![field("estado")],
            vec![
                ActionableError::new("pedido_no_encontrado", "corregir el identificador").unwrap(),
            ],
            vec![PrivateDetail::new("la tabla de almacenamiento").unwrap()],
        )
        .unwrap();

        assert_eq!(contract.capability(), "consultar un pedido");
        assert_eq!(contract.errors()[0].code(), "pedido_no_encontrado");
        assert_eq!(
            contract.private_details()[0].description(),
            "la tabla de almacenamiento"
        );
    }

    #[test]
    fn rejects_a_contract_without_an_observable_output() {
        let error = ApiContract::new(
            "consultar un pedido",
            vec![field("pedido_id")],
            vec![],
            vec![],
            vec![],
        )
        .unwrap_err();

        assert_eq!(error, ContractError::MissingOutput);
    }

    #[test]
    fn rejects_duplicate_names_across_the_public_boundary() {
        let error = ApiContract::new(
            "actualizar un pedido",
            vec![field("estado")],
            vec![field("estado")],
            vec![],
            vec![],
        )
        .unwrap_err();

        assert_eq!(
            error,
            ContractError::DuplicateField {
                name: "estado".to_owned(),
            }
        );
    }
}
