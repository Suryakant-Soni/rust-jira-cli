use std::borrow::Borrow;

use test_utils::MockDB;

use super::*;
#[test]
fn create_epic_should_work() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let res = jira_handle.create_epic(epic.clone());
    assert!(res.is_ok());
    let saved_id = res.unwrap();
    //read the result back from mock db and verify it
    let db_state = jira_handle
        .database
        .read_db()
        .unwrap();
    assert_eq!(saved_id, db_state.last_item_id);
    assert_eq!(
        db_state
            .epics
            .get(&saved_id)
            .unwrap(),
        &epic
    );
}

#[test]
fn create_story_should_error_if_invalid_epic_id() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("name".to_owned(), "".to_owned());
    let res = jira_handle.create_story(story, 999);
    assert!(res.is_err())
}

#[test]
fn create_story_should_work() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("name".to_owned(), "".to_owned());
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    //get created epic_id
    let saved_epic_id = res_epic.unwrap();
    let res_story = jira_handle.create_story(story.clone(), saved_epic_id);

    //read db directly form mock db to get the saved story and then compare it
    let db_state = jira_handle
        .database
        .read_db()
        .unwrap();
    //get the last created story
    let story_from_db = db_state
        .stories
        .get(
            res_story
                .unwrap()
                .borrow(),
        );
    assert_eq!(story_from_db, Some(&story))
}

#[test]
fn delete_epic_should_error_if_invalid_epic_id() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    //get created epic_id
    let saved_epic_id = res_epic.unwrap();
    let res = jira_handle.delete_epic(saved_epic_id + 1);
    assert!(res.is_err())
}

#[test]
fn delete_epic_should_work() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    //get created epic_id
    let saved_epic_id = res_epic.unwrap();
    let res = jira_handle.delete_epic(saved_epic_id);
    assert!(res.is_ok());
    // get same epic id should fail
    let db_after_delete = jira_handle
        .database
        .read_db()
        .unwrap();
    let get_after_delete = db_after_delete
        .epics
        .get(&saved_epic_id);
    assert!(get_after_delete.is_none())
}

#[test]
fn delete_story_should_error_if_invalid_epic_id() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("name".to_owned(), "".to_owned());
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epic first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    //get created epic_id
    let saved_epic_id = res_epic.unwrap();
    //create story now
    let res_story = jira_handle.create_story(story, saved_epic_id);
    assert!(res_story.is_ok());
    // try to delete story with some other epic id
    let res_delete = jira_handle.delete_story(saved_epic_id + 1, res_story.unwrap());
    assert!(res_delete.is_err());
}

#[test]
fn delete_story_should_error_if_story_not_found_in_epic() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("name".to_owned(), "".to_owned());
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    //get created epic_id
    let saved_epic_id = res_epic.unwrap();
    let res_story = jira_handle.create_story(story, saved_epic_id);
    assert!(res_story.is_ok());
    let res_story_queried = jira_handle.delete_story(saved_epic_id, 999);
    assert!(res_story_queried.is_err());
}

#[test]
fn delete_story_should_work() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("name".to_owned(), "".to_owned());
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    //get created epic_id
    let saved_epic_id = res_epic.unwrap();
    let res_story = jira_handle.create_story(story, saved_epic_id);
    assert!(res_story.is_ok());
    let created_story_id = res_story.unwrap();
    let res_story_queried = jira_handle.delete_story(saved_epic_id, created_story_id);
    assert!(res_story_queried.is_ok());
    let db_state = jira_handle
        .database
        .read_db()
        .unwrap();
    assert_eq!(db_state.last_item_id, 2);
    assert!(!db_state
        .epics
        .get(&saved_epic_id)
        .unwrap()
        .stories
        .contains(&created_story_id));
    assert_eq!(
        db_state
            .stories
            .get(&created_story_id),
        None
    );
}

#[test]
fn update_epic_status_should_error_if_invalid_epic_id() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    assert!(jira_handle
        .update_epic_status(2, Status::Closed)
        .is_err());
}
#[test]
fn update_epic_status_should_work() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    let epic_id = res_epic.unwrap();
    assert!(jira_handle
        .update_epic_status(epic_id, Status::Closed)
        .is_ok());
    // instead of api check directly via db low level connect
    assert_eq!(
        jira_handle
            .database
            .read_db()
            .unwrap()
            .epics
            .get(&epic_id)
            .unwrap()
            .status,
        Status::Closed
    );
}

#[test]
fn update_story_status_should_work() {
    let jira_handle = JiraHandle {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("name".to_owned(), "".to_owned());
    let epic = Epic::new("".to_owned(), "".to_owned());
    //create epi first
    let res_epic = jira_handle.create_epic(epic);
    assert!(res_epic.is_ok());
    //get created epic_id
    let saved_epic_id = res_epic.unwrap();
    let res_story = jira_handle.create_story(story, saved_epic_id);
    assert!(res_story.is_ok());
    let created_story_id = res_story.unwrap();
    let res_story_updated = jira_handle.update_story_status(created_story_id, Status::Resolved);
    assert!(res_story_updated.is_ok());
    let db_state = jira_handle
        .database
        .read_db()
        .unwrap();
    assert_eq!(
        db_state
            .stories
            .get(&created_story_id)
            .unwrap()
            .status,
        Status::Resolved
    );
}
