//! Modelo mínimo para elegir un estilo de API por su interacción observable.

use std::fmt;

/// Estilo principal con el que un proveedor expone una capacidad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiStyle {
    Rest,
    GraphQl,
    Grpc,
}

/// Interacción que un consumidor necesita resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsumerInteraction {
    ResourceAccess,
    ComposedView,
    TypedServiceCall,
    Streaming,
}

/// Límite que hace controlable el costo de una interacción.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractLimit {
    RequestBoundary,
    QueryBudget { max_depth: u8, max_fields: u16 },
    StreamWindow { max_in_flight: u16 },
}

/// Elección de estilo, interacción y límite operativo que el contrato sostiene.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleSelection {
    style: ApiStyle,
    interaction: ConsumerInteraction,
    limit: ContractLimit,
}

impl StyleSelection {
    pub fn new(
        style: ApiStyle,
        interaction: ConsumerInteraction,
        limit: ContractLimit,
    ) -> Result<Self, StyleSelectionError> {
        if !style.supports(interaction) {
            return Err(StyleSelectionError::UnsupportedInteraction { style, interaction });
        }
        validate_limit(style, interaction, limit)?;

        Ok(Self {
            style,
            interaction,
            limit,
        })
    }

    pub fn style(&self) -> ApiStyle {
        self.style
    }

    pub fn interaction(&self) -> ConsumerInteraction {
        self.interaction
    }

    pub fn limit(&self) -> ContractLimit {
        self.limit
    }
}

impl ApiStyle {
    pub fn supports(self, interaction: ConsumerInteraction) -> bool {
        matches!(
            (self, interaction),
            (Self::Rest, ConsumerInteraction::ResourceAccess)
                | (Self::GraphQl, ConsumerInteraction::ComposedView)
                | (
                    Self::Grpc,
                    ConsumerInteraction::TypedServiceCall | ConsumerInteraction::Streaming
                )
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StyleSelectionError {
    UnsupportedInteraction {
        style: ApiStyle,
        interaction: ConsumerInteraction,
    },
    MissingRequiredLimit {
        style: ApiStyle,
        interaction: ConsumerInteraction,
    },
    InvalidQueryBudget {
        max_depth: u8,
        max_fields: u16,
    },
    InvalidStreamWindow {
        max_in_flight: u16,
    },
}

impl fmt::Display for StyleSelectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedInteraction { .. } => {
                write!(
                    formatter,
                    "El estilo no corresponde a la interacción declarada."
                )
            }
            Self::MissingRequiredLimit { .. } => {
                write!(
                    formatter,
                    "La interacción necesita un límite operativo explícito."
                )
            }
            Self::InvalidQueryBudget {
                max_depth,
                max_fields,
            } => write!(
                formatter,
                "El presupuesto de consulta debe ser positivo; profundidad: {max_depth}, campos: {max_fields}."
            ),
            Self::InvalidStreamWindow { max_in_flight } => write!(
                formatter,
                "La ventana de stream debe permitir al menos un mensaje; recibió {max_in_flight}."
            ),
        }
    }
}

impl std::error::Error for StyleSelectionError {}

fn validate_limit(
    style: ApiStyle,
    interaction: ConsumerInteraction,
    limit: ContractLimit,
) -> Result<(), StyleSelectionError> {
    match (style, interaction, limit) {
        (
            ApiStyle::GraphQl,
            ConsumerInteraction::ComposedView,
            ContractLimit::QueryBudget {
                max_depth,
                max_fields,
            },
        ) if max_depth > 0 && max_fields > 0 => Ok(()),
        (
            ApiStyle::GraphQl,
            ConsumerInteraction::ComposedView,
            ContractLimit::QueryBudget {
                max_depth,
                max_fields,
            },
        ) => Err(StyleSelectionError::InvalidQueryBudget {
            max_depth,
            max_fields,
        }),
        (
            ApiStyle::Grpc,
            ConsumerInteraction::Streaming,
            ContractLimit::StreamWindow { max_in_flight },
        ) if max_in_flight > 0 => Ok(()),
        (
            ApiStyle::Grpc,
            ConsumerInteraction::Streaming,
            ContractLimit::StreamWindow { max_in_flight },
        ) => Err(StyleSelectionError::InvalidStreamWindow { max_in_flight }),
        (ApiStyle::Rest, ConsumerInteraction::ResourceAccess, ContractLimit::RequestBoundary)
        | (ApiStyle::Grpc, ConsumerInteraction::TypedServiceCall, ContractLimit::RequestBoundary) => {
            Ok(())
        }
        (style, interaction, _) => {
            Err(StyleSelectionError::MissingRequiredLimit { style, interaction })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApiStyle, ConsumerInteraction, ContractLimit, StyleSelection, StyleSelectionError,
    };

    #[test]
    fn selects_graphql_for_a_bounded_composed_view() {
        let selection = StyleSelection::new(
            ApiStyle::GraphQl,
            ConsumerInteraction::ComposedView,
            ContractLimit::QueryBudget {
                max_depth: 4,
                max_fields: 40,
            },
        )
        .unwrap();

        assert_eq!(selection.style(), ApiStyle::GraphQl);
        assert_eq!(selection.interaction(), ConsumerInteraction::ComposedView);
    }

    #[test]
    fn selects_grpc_streaming_with_a_flow_window() {
        let selection = StyleSelection::new(
            ApiStyle::Grpc,
            ConsumerInteraction::Streaming,
            ContractLimit::StreamWindow { max_in_flight: 16 },
        )
        .unwrap();

        assert_eq!(
            selection.limit(),
            ContractLimit::StreamWindow { max_in_flight: 16 }
        );
    }

    #[test]
    fn rejects_graphql_without_a_query_budget() {
        let error = StyleSelection::new(
            ApiStyle::GraphQl,
            ConsumerInteraction::ComposedView,
            ContractLimit::RequestBoundary,
        )
        .unwrap_err();

        assert_eq!(
            error,
            StyleSelectionError::MissingRequiredLimit {
                style: ApiStyle::GraphQl,
                interaction: ConsumerInteraction::ComposedView,
            }
        );
    }

    #[test]
    fn rejects_an_interaction_that_does_not_match_its_style() {
        let error = StyleSelection::new(
            ApiStyle::Rest,
            ConsumerInteraction::Streaming,
            ContractLimit::StreamWindow { max_in_flight: 16 },
        )
        .unwrap_err();

        assert_eq!(
            error,
            StyleSelectionError::UnsupportedInteraction {
                style: ApiStyle::Rest,
                interaction: ConsumerInteraction::Streaming,
            }
        );
    }
}
