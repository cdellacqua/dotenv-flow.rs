use std::fs::File;
use std::io::prelude::*;
use std::{env, io};
use tempfile::{tempdir, TempDir};

fn write_flush(dir: &TempDir, path: &str, content: &str) -> io::Result<()> {
    let dotenv_path = dir.path().join(path);
    let mut dotenv_file = File::create(dotenv_path)?;
    dotenv_file.write_all(content.as_bytes())?;
    dotenv_file.sync_all()?;

    Ok(())
}

fn optional_write_flush(dir: &TempDir, path: &str, content: Option<&str>) -> io::Result<()> {
    match content {
        None => (),
        Some(content) => write_flush(dir, path, content)?,
    }
    Ok(())
}

pub fn tempdir_with_dotenv(dotenv_text: &str) -> io::Result<TempDir> {
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;

    write_flush(&dir, ".env", dotenv_text)?;

    Ok(dir)
}

pub fn tempdir_with_dotenv_flow(
    dotenv_local_text: Option<&str>,
    dotenv_env_local_text: Option<&str>,
    dotenv_env_text: Option<&str>,
    dotenv_text: Option<&str>,
) -> io::Result<TempDir> {
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;

    optional_write_flush(&dir, ".env.local", dotenv_local_text)?;
    optional_write_flush(
        &dir,
        &format!(".env.{}.local", env::var("DOTENV_ENV").unwrap()),
        dotenv_env_local_text,
    )?;
    optional_write_flush(
        &dir,
        &format!(".env.{}", env::var("DOTENV_ENV").unwrap()),
        dotenv_env_text,
    )?;
    optional_write_flush(&dir, ".env", dotenv_text)?;

    Ok(dir)
}

pub fn make_test_dotenv() -> io::Result<TempDir> {
    tempdir_with_dotenv("TESTKEY=test_val")
}
