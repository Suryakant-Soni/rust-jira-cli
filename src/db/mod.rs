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
        self.database
            .read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut db_state = self.read_full_record()?;
        db_state
            .epics
            .insert(db_state.last_item_id + 1, epic)
            .ok_or_else(|| anyhow::Error::msg("error creating epic"));
        db_state.last_item_id += 1;
        self.database
            .write_db(&db_state);
        Ok(db_state.last_item_id)
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
        db_state.last_item_id += 1;
        self.database
            .write_db(&db_state);
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
            db_state
                .stories
                .remove(story_id);
        }
        db_state
            .epics
            .remove(&epic_id)
            .ok_or_else(|| anyhow::Error::msg("error while deleting epic"));
        self.database
            .write_db(&db_state);
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut db_state = self.read_full_record()?;
        let epic_mut = db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic with id {epic_id}"))?;
        let story_index = epic_mut
            .stories
            .iter()
            .position(|id| id == &story_id)
            .ok_or_else(|| anyhow!("story is not found in epic stories vector"))?;
        //remove the story id from epic object
        epic_mut
            .stories
            .remove(story_index);
        // remove story object form story hashmap
        db_state
            .stories
            .remove(&story_id);
        self.database
            .write_db(&db_state);
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_full_record()?;
        db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic with id {epic_id}"))?
            .status = status;
        self.database
            .write_db(&db_state);
        Ok(())
    }
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_full_record()?;
        db_state
            .stories
            .get_mut(&story_id)
            .ok_or_else(|| anyhow!("could not find story with id {story_id}"))?
            .status = status;
        self.database
            .write_db(&db_state);
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
mod tests {
    use super::*;
    mod db_tests; // tests for db reads and writes
    mod jira_crud_tests;
    mod test_utils; // utility for testing purpose // test for crud operation from server layer
}
