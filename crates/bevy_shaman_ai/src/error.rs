use std::fmt;
use std::error::Error;

/// AI and LLM system errors
#[derive(Debug, Clone)]
pub enum AIError {
    /// LLM model not loaded
    ModelNotLoaded {
        model_path: String,
    },

    /// LLM inference failed
    InferenceFailed {
        prompt: String,
        reason: String,
    },

    /// Response generation timeout
    GenerationTimeout {
        timeout_seconds: u64,
    },

    /// Invalid prompt or context
    InvalidPrompt {
        reason: String,
    },

    /// Response parsing failed
    ResponseParsingFailed {
        response: String,
        reason: String,
    },

    /// Cache operation failed
    CacheFailed {
        operation: String,
        reason: String,
    },

    /// Model configuration error
    ConfigurationError {
        parameter: String,
        reason: String,
    },

    /// NPC dialogue generation failed
    DialogueGenerationFailed {
        npc_id: String,
        reason: String,
    },

    /// Quest text generation failed
    QuestGenerationFailed {
        quest_id: String,
        reason: String,
    },

    /// Context size exceeded
    ContextSizeExceeded {
        max_size: usize,
        actual_size: usize,
    },

    /// Serialization/deserialization error
    SerializationError {
        reason: String,
    },

    /// Network or connection error (for remote APIs)
    ConnectionError {
        endpoint: String,
        reason: String,
    },

    /// Generic AI system error
    SystemError(String),
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIError::ModelNotLoaded { model_path } => {
                write!(f, "LLM model not loaded: {}", model_path)
            }
            AIError::InferenceFailed { prompt, reason } => {
                let prompt_preview = if prompt.len() > 50 {
                    format!("{}...", &prompt[..50])
                } else {
                    prompt.clone()
                };
                write!(f, "LLM inference failed for prompt '{}': {}", prompt_preview, reason)
            }
            AIError::GenerationTimeout { timeout_seconds } => {
                write!(f, "LLM generation timeout after {} seconds", timeout_seconds)
            }
            AIError::InvalidPrompt { reason } => {
                write!(f, "Invalid prompt: {}", reason)
            }
            AIError::ResponseParsingFailed { response, reason } => {
                let response_preview = if response.len() > 100 {
                    format!("{}...", &response[..100])
                } else {
                    response.clone()
                };
                write!(
                    f,
                    "Failed to parse LLM response '{}': {}",
                    response_preview, reason
                )
            }
            AIError::CacheFailed { operation, reason } => {
                write!(f, "Cache operation '{}' failed: {}", operation, reason)
            }
            AIError::ConfigurationError { parameter, reason } => {
                write!(f, "Configuration error for parameter '{}': {}", parameter, reason)
            }
            AIError::DialogueGenerationFailed { npc_id, reason } => {
                write!(f, "Failed to generate dialogue for NPC '{}': {}", npc_id, reason)
            }
            AIError::QuestGenerationFailed { quest_id, reason } => {
                write!(f, "Failed to generate quest text for '{}': {}", quest_id, reason)
            }
            AIError::ContextSizeExceeded { max_size, actual_size } => {
                write!(
                    f,
                    "Context size exceeded: max {} tokens, got {} tokens",
                    max_size, actual_size
                )
            }
            AIError::SerializationError { reason } => {
                write!(f, "Serialization error: {}", reason)
            }
            AIError::ConnectionError { endpoint, reason } => {
                write!(f, "Connection error to '{}': {}", endpoint, reason)
            }
            AIError::SystemError(msg) => {
                write!(f, "AI system error: {}", msg)
            }
        }
    }
}

impl Error for AIError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<serde_json::Error> for AIError {
    fn from(err: serde_json::Error) -> Self {
        AIError::SerializationError {
            reason: err.to_string(),
        }
    }
}

/// Result type for AI operations
pub type AIResult<T> = Result<T, AIError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = AIError::ModelNotLoaded {
            model_path: "/path/to/model.gguf".to_string(),
        };

        assert_eq!(
            error.to_string(),
            "LLM model not loaded: /path/to/model.gguf"
        );
    }

    #[test]
    fn test_context_size_error() {
        let error = AIError::ContextSizeExceeded {
            max_size: 2048,
            actual_size: 3000,
        };

        assert!(error.to_string().contains("3000 tokens"));
    }

    #[test]
    fn test_generation_timeout() {
        let error = AIError::GenerationTimeout {
            timeout_seconds: 30,
        };

        assert_eq!(
            error.to_string(),
            "LLM generation timeout after 30 seconds"
        );
    }
}
