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
        tokio::time::sleep(Duration::from_secs(10)).await;

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
