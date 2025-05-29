use ts_rs::TS;

mod different_types {
    use super::*;

    #[derive(TS)]
    #[ts(export, export_to = "type_conflict/different.ts")]
    pub struct MyType {
        field1: String,
    }

    #[derive(TS)]
    #[ts(export, export_to = "type_conflict/different.ts")]
    pub struct MyType {
        field2: i32,
    }

    #[test]
    fn test_different_types() {
        let result = MyType::export_all();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ts_rs::ExportError::TypeNameConflict(_)
        ));
    }
}

mod identical_types {
    use super::*;

    #[derive(TS)]
    #[ts(export, export_to = "type_conflict/identical.ts")]
    pub struct MyType {
        field1: String,
        field2: i32,
    }

    #[derive(TS)]
    #[ts(export, export_to = "type_conflict/identical.ts")]
    pub struct MyType {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_identical_types() {
        let result = MyType::export_all();
        assert!(result.is_ok(), "Exporting identical types should succeed");
    }
} 