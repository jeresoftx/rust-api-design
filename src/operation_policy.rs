//! Modelo mínimo de una política operativa observable para una capacidad.

use std::fmt;

/// Alcance y frescura de una respuesta que puede conservarse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePolicy {
    NoStore,
    Private { max_age_seconds: u32 },
    Shared { max_age_seconds: u32 },
}

/// Presupuesto recuperable de solicitudes para una ventana temporal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    max_requests: u32,
    window_seconds: u32,
    retry_after_seconds: u32,
}

impl RateLimit {
    pub fn new(
        max_requests: u32,
        window_seconds: u32,
        retry_after_seconds: u32,
    ) -> Result<Self, OperationPolicyError> {
        if max_requests == 0 || window_seconds == 0 || retry_after_seconds == 0 {
            return Err(OperationPolicyError::InvalidRateLimit {
                max_requests,
                window_seconds,
                retry_after_seconds,
            });
        }

        Ok(Self {
            max_requests,
            window_seconds,
            retry_after_seconds,
        })
    }

    pub fn retry_after_seconds(&self) -> u32 {
        self.retry_after_seconds
    }
}

/// Semántica que determina si una operación puede repetirse automáticamente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationSafety {
    ReadOnly,
    Idempotent,
    NonIdempotent,
}

/// Regla de reintento ante una falla transitoria.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryRule {
    Never,
    OnTransientFailure,
}

/// Política que el consumidor puede observar cuando usa una capacidad de API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationPolicy {
    cache: CachePolicy,
    rate_limit: RateLimit,
    safety: OperationSafety,
    retry: RetryRule,
}

impl OperationPolicy {
    pub fn new(
        cache: CachePolicy,
        rate_limit: RateLimit,
        safety: OperationSafety,
        retry: RetryRule,
    ) -> Result<Self, OperationPolicyError> {
        validate_cache(cache)?;
        if safety == OperationSafety::NonIdempotent && retry == RetryRule::OnTransientFailure {
            return Err(OperationPolicyError::UnsafeRetry { safety });
        }

        Ok(Self {
            cache,
            rate_limit,
            safety,
            retry,
        })
    }

    pub fn cache(&self) -> CachePolicy {
        self.cache
    }

    pub fn rate_limit(&self) -> &RateLimit {
        &self.rate_limit
    }

    pub fn retry(&self) -> RetryRule {
        self.retry
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationPolicyError {
    InvalidCacheFreshness {
        max_age_seconds: u32,
    },
    InvalidRateLimit {
        max_requests: u32,
        window_seconds: u32,
        retry_after_seconds: u32,
    },
    UnsafeRetry {
        safety: OperationSafety,
    },
}

impl fmt::Display for OperationPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCacheFreshness { max_age_seconds } => write!(
                formatter,
                "La frescura de cache debe ser positiva; recibió {max_age_seconds}."
            ),
            Self::InvalidRateLimit { .. } => write!(
                formatter,
                "El límite debe declarar presupuesto, ventana y espera positivos."
            ),
            Self::UnsafeRetry { .. } => write!(
                formatter,
                "Una operación no idempotente no puede reintentarse automáticamente."
            ),
        }
    }
}

impl std::error::Error for OperationPolicyError {}

fn validate_cache(cache: CachePolicy) -> Result<(), OperationPolicyError> {
    match cache {
        CachePolicy::NoStore => Ok(()),
        CachePolicy::Private { max_age_seconds } | CachePolicy::Shared { max_age_seconds }
            if max_age_seconds > 0 =>
        {
            Ok(())
        }
        CachePolicy::Private { max_age_seconds } | CachePolicy::Shared { max_age_seconds } => {
            Err(OperationPolicyError::InvalidCacheFreshness { max_age_seconds })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CachePolicy, OperationPolicy, OperationPolicyError, OperationSafety, RateLimit, RetryRule,
    };

    #[test]
    fn declares_freshness_limit_and_safe_retry_for_a_catalog() {
        let rate_limit = RateLimit::new(120, 60, 10).unwrap();
        let policy = OperationPolicy::new(
            CachePolicy::Shared {
                max_age_seconds: 300,
            },
            rate_limit,
            OperationSafety::ReadOnly,
            RetryRule::OnTransientFailure,
        )
        .unwrap();

        assert_eq!(policy.retry(), RetryRule::OnTransientFailure);
        assert_eq!(policy.rate_limit().retry_after_seconds(), 10);
    }

    #[test]
    fn rejects_cache_without_positive_freshness() {
        let rate_limit = RateLimit::new(120, 60, 10).unwrap();
        let error = OperationPolicy::new(
            CachePolicy::Shared { max_age_seconds: 0 },
            rate_limit,
            OperationSafety::ReadOnly,
            RetryRule::OnTransientFailure,
        )
        .unwrap_err();

        assert_eq!(
            error,
            OperationPolicyError::InvalidCacheFreshness { max_age_seconds: 0 }
        );
    }

    #[test]
    fn rejects_retry_for_a_non_idempotent_operation() {
        let rate_limit = RateLimit::new(30, 60, 20).unwrap();
        let error = OperationPolicy::new(
            CachePolicy::NoStore,
            rate_limit,
            OperationSafety::NonIdempotent,
            RetryRule::OnTransientFailure,
        )
        .unwrap_err();

        assert_eq!(
            error,
            OperationPolicyError::UnsafeRetry {
                safety: OperationSafety::NonIdempotent,
            }
        );
    }
}
