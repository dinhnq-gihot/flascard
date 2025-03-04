use {
    sea_orm::{ConnectOptions, Database as DB, DatabaseConnection, DbErr},
    std::time::Duration,
};

pub struct Database {
    db: DatabaseConnection,
}

impl Database {
    pub async fn try_new(connection_string: &str) -> Result<Self, DbErr> {
        let mut opt = ConnectOptions::new(connection_string);
        opt.max_connections(10) // ✅ Set max number of connections
            .min_connections(2) // ✅ Keep some idle connections
            .connect_timeout(Duration::from_secs(10)) // ✅ Connection timeout
            .acquire_timeout(Duration::from_secs(5)) // ✅ Timeout for acquiring connection
            .idle_timeout(Duration::from_secs(600)); // ✅ Close idle connections after 10 mins

        let db = DB::connect(opt).await?;

        Ok(Self { db })
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        self.db.clone()
    }
}
