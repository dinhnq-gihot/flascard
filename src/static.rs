use {
    crate::{debug, enums::error::*},
    once_cell::sync::Lazy,
    parking_lot::Mutex,
    std::{
        fs::{self, OpenOptions},
        io::Write,
        path::PathBuf,
        time::Duration,
    },
};

pub static BLACKLIST_TOKEN_VEC: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn init_blacklist_jwt(path: &PathBuf) -> Result<()> {
    let contents = fs::read_to_string(path).map_err(|e| Error::Anyhow(e.into()))?;
    let tokens =
        serde_json::from_str::<Vec<String>>(&contents).map_err(|e| Error::Anyhow(e.into()))?;

    debug!("init_blacklist_jwt: {tokens:?}");
    *BLACKLIST_TOKEN_VEC.lock() = tokens;

    Ok(())
}

pub async fn write_blacklist_jwt(path: &PathBuf) -> Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;

        let tokens = BLACKLIST_TOKEN_VEC.lock().clone();
        let json = serde_json::to_string_pretty(&tokens).map_err(|e| Error::Anyhow(e.into()))?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| Error::Anyhow(e.into()))?;
        file.write_all(json.as_bytes())
            .map_err(|e| Error::Anyhow(e.into()))?;

        debug!("write_blacklist_jwt: {path:?}")
    }
}

#[cfg(test)]
mod tests {
    use {
        serde::{Deserialize, Serialize},
        serde_json::Value,
    };

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Item {
        text: String,
        is_correct: bool, // Use `bool` instead of `String` for correctness
    }

    #[test]
    fn test_json() {
        let json1 = r#"[{"text": "a", "is_correct": true}, {"text": "b", "is_correct": false}]"#;
        let json2 = r#"[{"text": "b", "is_correct": false}, {"text": "a", "is_correct": true}]"#;

        let obj1: Value = serde_json::from_str(json1).unwrap();
        let obj2: Value = serde_json::from_str(json2).unwrap();

        // Compare the two vectors
        if obj1 == obj2 {
            println!("✅ The JSON arrays are equal!");
        } else {
            println!("❌ The JSON arrays are different!");
        }
    }
}
