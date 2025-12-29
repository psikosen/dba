use anyhow::{anyhow, Result};
use std::fmt;
use url::Url;

/// Service configuration errors
#[derive(Debug)]
pub enum ConfigError {
    InvalidUrl(String, url::ParseError),
    InvalidScheme { env_var: String, expected: Vec<String>, got: String },
    MissingRequired(String),
    WeakPassword { env_var: String, reason: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidUrl(env_var, err) => {
                write!(f, "Invalid URL for {}: {}", env_var, err)
            }
            ConfigError::InvalidScheme { env_var, expected, got } => {
                write!(
                    f,
                    "Invalid scheme for {}: expected one of {:?}, got '{}'",
                    env_var, expected, got
                )
            }
            ConfigError::MissingRequired(env_var) => {
                write!(f, "Required environment variable {} is not set", env_var)
            }
            ConfigError::WeakPassword { env_var, reason } => {
                write!(f, "Weak password for {}: {}", env_var, reason)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Validated service configuration
#[derive(Clone, Debug)]
pub struct ServiceConfig {
    pub dragonfly_url: String,
    pub rabbitmq_url: String,
}

impl ServiceConfig {
    /// Create configuration from environment variables with validation
    ///
    /// # Security
    /// - Validates URL format to prevent injection attacks
    /// - Enforces proper URL schemes (redis/rediss, amqp/amqps)
    /// - Checks password strength in production
    /// - Fails fast on invalid configuration
    pub fn from_env() -> Result<Self> {
        // Get DragonflyDB URL
        let dragonfly_url = std::env::var("DRAGONFLY_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());

        // Validate DragonflyDB URL
        Self::validate_redis_url(&dragonfly_url, "DRAGONFLY_URL")?;

        // Get RabbitMQ URL
        let rabbitmq_url = std::env::var("RABBITMQ_URL")
            .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".to_string());

        // Validate RabbitMQ URL
        Self::validate_amqp_url(&rabbitmq_url, "RABBITMQ_URL")?;

        // In production, validate password strength
        if std::env::var("ENVIRONMENT").unwrap_or_default() == "production" {
            Self::validate_production_credentials(&dragonfly_url, &rabbitmq_url)?;
        }

        Ok(Self {
            dragonfly_url,
            rabbitmq_url,
        })
    }

    /// Validate Redis/DragonflyDB URL
    fn validate_redis_url(url_str: &str, env_var: &str) -> Result<()> {
        // Parse URL
        let url = Url::parse(url_str)
            .map_err(|e| ConfigError::InvalidUrl(env_var.to_string(), e))?;

        // Validate scheme
        let scheme = url.scheme();
        if !["redis", "rediss"].contains(&scheme) {
            return Err(ConfigError::InvalidScheme {
                env_var: env_var.to_string(),
                expected: vec!["redis".to_string(), "rediss".to_string()],
                got: scheme.to_string(),
            }
            .into());
        }

        // Validate host exists
        if url.host_str().is_none() {
            return Err(anyhow!(
                "Invalid {}: missing hostname",
                env_var
            ));
        }

        Ok(())
    }

    /// Validate AMQP/RabbitMQ URL
    fn validate_amqp_url(url_str: &str, env_var: &str) -> Result<()> {
        // Parse URL
        let url = Url::parse(url_str)
            .map_err(|e| ConfigError::InvalidUrl(env_var.to_string(), e))?;

        // Validate scheme
        let scheme = url.scheme();
        if !["amqp", "amqps"].contains(&scheme) {
            return Err(ConfigError::InvalidScheme {
                env_var: env_var.to_string(),
                expected: vec!["amqp".to_string(), "amqps".to_string()],
                got: scheme.to_string(),
            }
            .into());
        }

        // Validate host exists
        if url.host_str().is_none() {
            return Err(anyhow!(
                "Invalid {}: missing hostname",
                env_var
            ));
        }

        Ok(())
    }

    /// Validate production credentials are strong
    fn validate_production_credentials(dragonfly_url: &str, rabbitmq_url: &str) -> Result<()> {
        // Parse URLs to extract passwords
        let dragonfly_parsed = Url::parse(dragonfly_url).ok();
        let rabbitmq_parsed = Url::parse(rabbitmq_url).ok();

        // Check DragonflyDB password
        if let Some(url) = dragonfly_parsed {
            if let Some(password) = url.password() {
                Self::validate_password_strength(password, "DRAGONFLY_PASSWORD")?;
            }
        }

        // Check RabbitMQ password
        if let Some(url) = rabbitmq_parsed {
            if let Some(password) = url.password() {
                Self::validate_password_strength(password, "RABBITMQ_PASSWORD")?;
            }
        }

        Ok(())
    }

    /// Validate password strength
    fn validate_password_strength(password: &str, env_var: &str) -> Result<()> {
        // Check for common weak passwords
        const WEAK_PASSWORDS: &[&str] = &[
            "changeme",
            "password",
            "123456",
            "admin",
            "guest",
            "root",
            "test",
            "default",
        ];

        if WEAK_PASSWORDS.contains(&password) {
            return Err(ConfigError::WeakPassword {
                env_var: env_var.to_string(),
                reason: format!("Common weak password '{}' not allowed in production", password),
            }
            .into());
        }

        // Minimum length check
        if password.len() < 16 {
            return Err(ConfigError::WeakPassword {
                env_var: env_var.to_string(),
                reason: format!(
                    "Password too short ({} chars, minimum 16 required for production)",
                    password.len()
                ),
            }
            .into());
        }

        Ok(())
    }

    /// Validate TLS is enabled for production
    pub fn ensure_tls_enabled(&self) -> Result<()> {
        if std::env::var("ENVIRONMENT").unwrap_or_default() != "production" {
            return Ok(()); // Skip in non-production
        }

        // Check DragonflyDB uses TLS
        if self.dragonfly_url.starts_with("redis://") {
            return Err(anyhow!(
                "Production deployment requires TLS for DragonflyDB (use rediss:// instead of redis://)"
            ));
        }

        // Check RabbitMQ uses TLS
        if self.rabbitmq_url.starts_with("amqp://") {
            return Err(anyhow!(
                "Production deployment requires TLS for RabbitMQ (use amqps:// instead of amqp://)"
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_redis_url() {
        assert!(ServiceConfig::validate_redis_url("redis://localhost:6379", "TEST").is_ok());
        assert!(ServiceConfig::validate_redis_url("rediss://localhost:6380", "TEST").is_ok());
        assert!(ServiceConfig::validate_redis_url(
            "redis://:password@host:6379",
            "TEST"
        )
        .is_ok());
    }

    #[test]
    fn test_invalid_redis_scheme() {
        let result = ServiceConfig::validate_redis_url("http://localhost:6379", "TEST");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_amqp_url() {
        assert!(
            ServiceConfig::validate_amqp_url("amqp://guest:guest@localhost:5672", "TEST").is_ok()
        );
        assert!(ServiceConfig::validate_amqp_url(
            "amqps://user:pass@host:5671/%2f",
            "TEST"
        )
        .is_ok());
    }

    #[test]
    fn test_invalid_amqp_scheme() {
        let result = ServiceConfig::validate_amqp_url("http://localhost:5672", "TEST");
        assert!(result.is_err());
    }

    #[test]
    fn test_weak_password_detection() {
        assert!(ServiceConfig::validate_password_strength("changeme", "TEST").is_err());
        assert!(ServiceConfig::validate_password_strength("password", "TEST").is_err());
        assert!(ServiceConfig::validate_password_strength("guest", "TEST").is_err());
    }

    #[test]
    fn test_short_password_detection() {
        assert!(ServiceConfig::validate_password_strength("short123", "TEST").is_err());
    }

    #[test]
    fn test_strong_password_accepted() {
        assert!(ServiceConfig::validate_password_strength(
            "aB3$xY9#mK2@pL5&qR8!",
            "TEST"
        )
        .is_ok());
    }
}
