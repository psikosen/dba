use std::fmt;
use std::error::Error;

/// Combat system errors
#[derive(Debug, Clone)]
pub enum CombatError {
    /// Entity does not have required combat components
    MissingComponent {
        entity: String,
        component: String,
    },

    /// Invalid attack timing or rhythm
    InvalidTiming {
        expected: f32,
        actual: f32,
        tolerance: f32,
    },

    /// Attack failed due to invalid state
    InvalidAttackState {
        entity: String,
        state: String,
        reason: String,
    },

    /// Damage calculation failed
    DamageCalculationFailed {
        reason: String,
    },

    /// Status effect application failed
    StatusEffectFailed {
        effect: String,
        target: String,
        reason: String,
    },

    /// Combat state transition error
    StateTransitionFailed {
        from: String,
        to: String,
        reason: String,
    },

    /// Resource (spirit/stamina) insufficient
    InsufficientResource {
        resource: String,
        required: f32,
        available: f32,
    },

    /// Weapon or ability not found
    WeaponNotFound {
        weapon_id: String,
    },

    /// Combo system error
    ComboError {
        reason: String,
    },

    /// Generic combat system error
    SystemError(String),
}

impl fmt::Display for CombatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CombatError::MissingComponent { entity, component } => {
                write!(f, "Entity '{}' is missing required component '{}'", entity, component)
            }
            CombatError::InvalidTiming { expected, actual, tolerance } => {
                write!(
                    f,
                    "Invalid timing: expected {:.3}s, got {:.3}s (tolerance: {:.3}s)",
                    expected, actual, tolerance
                )
            }
            CombatError::InvalidAttackState { entity, state, reason } => {
                write!(
                    f,
                    "Entity '{}' cannot attack in state '{}': {}",
                    entity, state, reason
                )
            }
            CombatError::DamageCalculationFailed { reason } => {
                write!(f, "Damage calculation failed: {}", reason)
            }
            CombatError::StatusEffectFailed { effect, target, reason } => {
                write!(
                    f,
                    "Failed to apply status effect '{}' to '{}': {}",
                    effect, target, reason
                )
            }
            CombatError::StateTransitionFailed { from, to, reason } => {
                write!(
                    f,
                    "Combat state transition from '{}' to '{}' failed: {}",
                    from, to, reason
                )
            }
            CombatError::InsufficientResource { resource, required, available } => {
                write!(
                    f,
                    "Insufficient {}: required {:.1}, available {:.1}",
                    resource, required, available
                )
            }
            CombatError::WeaponNotFound { weapon_id } => {
                write!(f, "Weapon '{}' not found", weapon_id)
            }
            CombatError::ComboError { reason } => {
                write!(f, "Combo system error: {}", reason)
            }
            CombatError::SystemError(msg) => {
                write!(f, "Combat system error: {}", msg)
            }
        }
    }
}

impl Error for CombatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Result type for combat operations
pub type CombatResult<T> = Result<T, CombatError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = CombatError::InsufficientResource {
            resource: "Spirit".to_string(),
            required: 50.0,
            available: 30.0,
        };

        assert_eq!(
            error.to_string(),
            "Insufficient Spirit: required 50.0, available 30.0"
        );
    }

    #[test]
    fn test_invalid_timing_error() {
        let error = CombatError::InvalidTiming {
            expected: 1.0,
            actual: 1.5,
            tolerance: 0.2,
        };

        assert!(error.to_string().contains("Invalid timing"));
    }
}
