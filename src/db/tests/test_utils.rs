use super::{DBState, Database};
use anyhow::{Ok, Result};
use std::{cell::RefCell, collections::HashMap};

pub struct MockDB {
    pub state: RefCell<DBState>,
}

impl MockDB {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(DBState {
                last_item_id: 0,
                epics: HashMap::new(),
                stories: HashMap::new(),
            }),
        }
    }
}

impl Database for MockDB {
    fn read_db(&self) -> Result<DBState> {
        let st = self
            .state
            .borrow()
            .clone();
        Ok(st)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        *self
            .state
            .borrow_mut() = db_state.clone();
        Ok(())
    }
}
