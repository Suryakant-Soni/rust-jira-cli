use std::fs;

use crate::models::{DBState, Epic, Status, Story};
use anyhow::{anyhow, Error, Ok, Result};
use serde::{Deserialize, Serialize};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

// need struct to handle CRUD operation
pub struct JiraHandle {
    database: Box<dyn Database>,
}

impl JiraHandle {
    pub fn new(file_path: String) -> Self {
        let db = JSONFileDatabase::new(file_path);
        JiraHandle {
            database: Box::new(db),
        }
    }

    pub fn read_full_record(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut db_state = self.read_full_record()?;
        db_state
            .epics
            .insert(db_state.last_item_id + 1, epic)
            .ok_or_else(|| anyhow::Error::msg("error creating epic"));

        self.database.write_db(&db_state);
        Ok(db_state.last_item_id + 1)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut db_state = self.read_full_record()?;
        let new_id = db_state.last_item_id + 1;
        db_state
            .stories
            .insert(new_id, story)
            .ok_or_else(|| anyhow!("error while creating story"));
        db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database"))?
            .stories
            .push(new_id);
        self.database.write_db(&db_state);
        Ok(new_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut db_state = self.read_full_record()?;
        for story_id in &db_state
            .epics
            .get(&epic_id)
            .ok_or_else(|| anyhow!("error in finding epic {epic_id} in database"))?
            .stories
        {
            db_state.stories.remove(story_id);
        }
        db_state
            .epics
            .remove(&epic_id)
            .ok_or_else(|| anyhow::Error::msg("error while deleting epic"));
        self.database.write_db(&db_state);
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_full_record()?;
        db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic with id {epic_id}"))?
            .status = status;
        self.database.write_db(&db_state);
        Ok(())
    }
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_full_record()?;
        db_state
            .stories
            .get_mut(&story_id)
            .ok_or_else(|| anyhow!("could not find story with id {story_id}"))?
            .status = status;
        self.database.write_db(&db_state);
        Ok(())
    }
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl JSONFileDatabase {
    fn new(file_path: String) -> Self {
        JSONFileDatabase { file_path }
    }
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        // read the file from the path into string
        let file_str = fs::read_to_string(&self.file_path)?;

        //deserialze the json string into struct vessel
        let record: DBState = serde_json::from_str(&file_str)?;
        Ok(record)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let ser_string = serde_json::to_string(db_state)?;
        fs::write(&self.file_path, ser_string)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use crate::models::{Epic, Story};

        use super::*;
        use std::{collections::HashMap, io::Write};

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let json_file_db = JSONFileDatabase::new("Invalid path".to_owned());
            assert!(json_file_db.read_db().is_err());
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
            assert!(db.read_db().is_err())
        }

        #[test]
        fn read_db_should_pass_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();
            //we have used here to_owned instead of .expect method as in above test func
            let db = JSONFileDatabase::new(tmpfile.path().to_str().unwrap().to_owned());
            assert!(db.read_db().is_ok())
        }

        #[test]
        fn write_db_should_work() {
            let tmpfile = tempfile::NamedTempFile::new().unwrap();
            //create a db instance for above temp file connection
            let db = JSONFileDatabase::new(tmpfile.path().to_str().unwrap().to_string());
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
            assert!(db.write_db(&db_state).is_ok());
            let read_result = db.read_db().unwrap();
            assert_eq!(read_result, db_state);
        }
    }
}
