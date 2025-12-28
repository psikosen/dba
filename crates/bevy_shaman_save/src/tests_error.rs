/// Unit tests for SaveError and LoadError types
#[cfg(test)]
mod error_tests {
    use crate::error::{LoadError, SaveError};
    use std::error::Error;
    use std::io;

    // ============================================================================
    // SAVE ERROR TESTS
    // ============================================================================

    #[test]
    fn test_save_error_serialization_failed() {
        let error = SaveError::SerializationFailed("invalid JSON".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Failed to serialize save data"));
        assert!(display.contains("invalid JSON"));
    }

    #[test]
    fn test_save_error_directory_creation_failed() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let error = SaveError::DirectoryCreationFailed(io_error);
        let display = format!("{}", error);
        assert!(display.contains("Failed to create saves directory"));
    }

    #[test]
    fn test_save_error_write_failed() {
        let io_error = io::Error::new(io::ErrorKind::WriteZero, "disk full");
        let error = SaveError::WriteFailed(io_error);
        let display = format!("{}", error);
        assert!(display.contains("Failed to write save file"));
    }

    #[test]
    fn test_save_error_invalid_data() {
        let error = SaveError::InvalidData("corrupted save".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Invalid save data"));
        assert!(display.contains("corrupted save"));
    }

    #[test]
    fn test_save_error_implements_error_trait() {
        let error = SaveError::SerializationFailed("test".to_string());
        let _: &dyn Error = &error; // Should compile
    }

    #[test]
    fn test_save_error_source_chain() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "not found");
        let error = SaveError::DirectoryCreationFailed(io_error);
        assert!(error.source().is_some());
    }

    #[test]
    fn test_save_error_source_none_for_string_errors() {
        let error = SaveError::SerializationFailed("test".to_string());
        assert!(error.source().is_none());
    }

    #[test]
    fn test_save_error_from_serde_json() {
        let json = "{ invalid json }";
        let result: Result<serde_json::Value, _> = serde_json::from_str(json);
        if let Err(serde_err) = result {
            let save_error: SaveError = serde_err.into();
            let display = format!("{}", save_error);
            assert!(display.contains("Failed to serialize save data"));
        }
    }

    #[test]
    fn test_save_error_debug_impl() {
        let error = SaveError::InvalidData("test".to_string());
        let debug = format!("{:?}", error);
        assert!(debug.contains("InvalidData"));
    }

    // ============================================================================
    // LOAD ERROR TESTS
    // ============================================================================

    #[test]
    fn test_load_error_read_failed() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error = LoadError::ReadFailed(io_error);
        let display = format!("{}", error);
        assert!(display.contains("Failed to read save file"));
    }

    #[test]
    fn test_load_error_deserialization_failed() {
        let error = LoadError::DeserializationFailed("malformed JSON".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Failed to deserialize save data"));
        assert!(display.contains("malformed JSON"));
    }

    #[test]
    fn test_load_error_file_not_found() {
        let error = LoadError::FileNotFound("saves/slot1.json".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Save file not found"));
        assert!(display.contains("saves/slot1.json"));
    }

    #[test]
    fn test_load_error_incompatible_format() {
        let error = LoadError::IncompatibleFormat("version mismatch".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Save file format is incompatible"));
        assert!(display.contains("version mismatch"));
    }

    #[test]
    fn test_load_error_implements_error_trait() {
        let error = LoadError::FileNotFound("test.json".to_string());
        let _: &dyn Error = &error; // Should compile
    }

    #[test]
    fn test_load_error_source_chain() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let error = LoadError::ReadFailed(io_error);
        assert!(error.source().is_some());
    }

    #[test]
    fn test_load_error_source_none_for_string_errors() {
        let error = LoadError::FileNotFound("test.json".to_string());
        assert!(error.source().is_none());
    }

    #[test]
    fn test_load_error_from_serde_json() {
        let json = "{ invalid: true }";
        let result: Result<serde_json::Value, _> = serde_json::from_str(json);
        if let Err(serde_err) = result {
            let load_error: LoadError = serde_err.into();
            let display = format!("{}", load_error);
            assert!(display.contains("Failed to deserialize save data"));
        }
    }

    #[test]
    fn test_load_error_debug_impl() {
        let error = LoadError::IncompatibleFormat("v2".to_string());
        let debug = format!("{:?}", error);
        assert!(debug.contains("IncompatibleFormat"));
    }

    // ============================================================================
    // INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn test_error_types_are_distinct() {
        let save_err = SaveError::InvalidData("test".to_string());
        let load_err = LoadError::FileNotFound("test.json".to_string());

        // Verify they format differently
        let save_display = format!("{}", save_err);
        let load_display = format!("{}", load_err);

        assert_ne!(save_display, load_display);
    }

    #[test]
    fn test_all_save_error_variants() {
        let errors = vec![
            SaveError::SerializationFailed("test".to_string()),
            SaveError::DirectoryCreationFailed(io::Error::new(io::ErrorKind::Other, "test")),
            SaveError::WriteFailed(io::Error::new(io::ErrorKind::Other, "test")),
            SaveError::InvalidData("test".to_string()),
        ];

        assert_eq!(errors.len(), 4);
        for error in errors {
            // All should implement Display and Error
            let _ = format!("{}", error);
            let _ = error.source();
        }
    }

    #[test]
    fn test_all_load_error_variants() {
        let errors = vec![
            LoadError::ReadFailed(io::Error::new(io::ErrorKind::Other, "test")),
            LoadError::DeserializationFailed("test".to_string()),
            LoadError::FileNotFound("test.json".to_string()),
            LoadError::IncompatibleFormat("test".to_string()),
        ];

        assert_eq!(errors.len(), 4);
        for error in errors {
            // All should implement Display and Error
            let _ = format!("{}", error);
            let _ = error.source();
        }
    }
}
