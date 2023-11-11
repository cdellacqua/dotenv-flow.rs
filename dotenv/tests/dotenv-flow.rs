mod common;

use dotenv_flow::*;
use std::env;

use crate::common::*;

#[test]
fn test_dotenv_flow_with_only_local_file() {
    env::remove_var("TEST_FLOW_KEY");
    env::set_var("DOTENV_ENV", "test");
    let dir = tempdir_with_dotenv_flow(None, Some("TEST_FLOW_KEY=test_val_local"), None, None).unwrap();

    dotenv_flow().ok();
    assert_eq!(env::var("TEST_FLOW_KEY").unwrap(), "test_val_local");

    dir.close().unwrap();
}

#[test]
fn test_dotenv_flow_with_local_and_default_file() {
    env::remove_var("TEST_FLOW_KEY");
    env::set_var("DOTENV_ENV", "test");
    let dir = tempdir_with_dotenv_flow(
        None,
        Some("TEST_FLOW_KEY=test_val_local"),
        None,
        Some("TEST_FLOW_KEY=test_val_default"),
    )
    .unwrap();

    dotenv_flow().ok();
    assert_eq!(env::var("TEST_FLOW_KEY").unwrap(), "test_val_local");

    dir.close().unwrap();
}

#[test]
fn test_dotenv_flow_with_local_and_env_local_file() {
    env::remove_var("TEST_FLOW_KEY");
    env::set_var("DOTENV_ENV", "test");
    let dir = tempdir_with_dotenv_flow(
        Some("TEST_FLOW_KEY=test_val_env_local"),
        Some("TEST_FLOW_KEY=test_val_local"),
        None,
        None,
    )
    .unwrap();

    dotenv_flow().ok();
    assert_eq!(env::var("TEST_FLOW_KEY").unwrap(), "test_val_env_local");

    dir.close().unwrap();
}

#[test]
fn test_dotenv_flow_with_local_and_env_file() {
    env::remove_var("TEST_FLOW_KEY");
    env::set_var("DOTENV_ENV", "test");
    let dir = tempdir_with_dotenv_flow(
        None,
        Some("TEST_FLOW_KEY=test_val_local"),
        Some("TEST_FLOW_KEY=test_val_default"),
        None,
    )
    .unwrap();

    dotenv_flow().ok();
    assert_eq!(env::var("TEST_FLOW_KEY").unwrap(), "test_val_local");

    dir.close().unwrap();
}

#[test]
fn test_dotenv_flow_with_all_files() {
    env::remove_var("TEST_FLOW_KEY");
    env::set_var("DOTENV_ENV", "test");
    let dir = tempdir_with_dotenv_flow(
        Some("TEST_FLOW_KEY=test_val_env_local"),
        Some("TEST_FLOW_KEY=test_val_local"),
        Some("TEST_FLOW_KEY=test_val_env"),
        Some("TEST_FLOW_KEY=test_val_default"),
    )
    .unwrap();

    dotenv_flow().ok();
    assert_eq!(env::var("TEST_FLOW_KEY").unwrap(), "test_val_env_local");

    dir.close().unwrap();
}

#[test]
fn test_dotenv_flow_with_no_local() {
    env::remove_var("TEST_FLOW_KEY");
    env::set_var("DOTENV_ENV", "test");
    let dir = tempdir_with_dotenv_flow(
        None,
        None,
        Some("TEST_FLOW_KEY=test_val_env"),
        Some("TEST_FLOW_KEY=test_val_default"),
    )
    .unwrap();

    dotenv_flow().ok();
    assert_eq!(env::var("TEST_FLOW_KEY").unwrap(), "test_val_env");

    dir.close().unwrap();
}
