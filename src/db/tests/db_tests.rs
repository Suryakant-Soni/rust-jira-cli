use super::*;
mod database {
    use super::*;
    use crate::models::{Epic, Story};
    use std::{collections::HashMap, io::Write};

    #[test]
    fn read_db_should_fail_with_invalid_path() {
        let json_file_db = JSONFileDatabase::new("Invalid path".to_owned());
        assert!(json_file_db
            .read_db()
            .is_err());
    }
    #[test]
    fn read_db_should_fail_with_invalid_json() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
        write!(tmpfile, "{}", file_contents).unwrap();

        let db = JSONFileDatabase::new(
            tmpfile
                .path()
                .to_str()
                .expect("failed to convert tmpfile str")
                .to_string(),
        );
        assert!(db
            .read_db()
            .is_err())
    }

    #[test]
    fn read_db_should_pass_json_file() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

        let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
        write!(tmpfile, "{}", file_contents).unwrap();
        //we have used here to_owned instead of .expect method as in above test func
        let db = JSONFileDatabase::new(
            tmpfile
                .path()
                .to_str()
                .unwrap()
                .to_owned(),
        );
        assert!(db
            .read_db()
            .is_ok())
    }

    #[test]
    fn write_db_should_work() {
        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        //create a db instance for above temp file connection
        let db = JSONFileDatabase::new(
            tmpfile
                .path()
                .to_str()
                .unwrap()
                .to_string(),
        );
        let story = Story {
            name: "epic 1".to_owned(),
            description: "description 1".to_owned(),
            status: crate::models::Status::Open,
        };
        let epic = Epic {
            name: "epic 1".to_owned(),
            description: "description 1".to_owned(),
            status: crate::models::Status::Open,
            stories: vec![2],
        };
        let mut stories = HashMap::new();
        stories.insert(2, story);
        let mut epics = HashMap::new();
        epics.insert(1, epic);
        let db_state = DBState {
            last_item_id: 2,
            epics,
            stories,
        };
        assert!(db
            .write_db(&db_state)
            .is_ok());
        let read_result = db
            .read_db()
            .unwrap();
        assert_eq!(read_result, db_state);
    }
}
