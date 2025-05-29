use ts_rs::TS;
// NOTE: this file is not executed now, could not get the tests to work as the ts
// output is triggered regardless of calling export_all.
//
// However, I've verified that it does work by running this branch of ts-rs in my project.

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
