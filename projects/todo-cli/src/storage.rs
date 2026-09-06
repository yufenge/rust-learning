use serde::Serialize;
use serde::de::DeserializeOwned;

use std::fs::{self, OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;

const DATA_PATH: &str = "data/tasks.json";

pub fn save<T>(datas: &Vec<T>) -> Result<(), std::io::Error>
where
    T: Serialize,
{
    let path = Path::new(DATA_PATH);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .append(false)
        .open(path)?;

    let text = serde_json::to_string_pretty(datas)?;

    file.write_all(text.as_bytes())?;

    Ok(())
}

pub fn load<T>() -> Result<T, std::io::Error>
where
    T: DeserializeOwned,
{
    let text = read_to_string(DATA_PATH)?;

    let data: T = serde_json::from_str(&text)?;

    Ok(data)
}
