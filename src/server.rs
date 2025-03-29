use {
    crate::{
        config::Config,
        db::db_connection::Database,
        enums::error::{Error, Result},
        error,
        routes::setup_routing,
        services::{
            implements::{init_service_implements, ServiceImpls},
            traits::prelude::*,
        },
    },
    std::sync::Arc,
    tokio::net::TcpListener,
    tracing::info,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub set_service: Arc<dyn SetService>,
    pub qna_service: Arc<dyn QnAService>,
    // pub quiz_service: Arc<dyn QuizService>,
    // pub quiz_question_service: Arc<dyn QuizQuestionService>,
    // pub test_service: Arc<dyn TestService>,
}

impl AppState {
    pub async fn init(cfg: Config) -> Result<Self> {
        let db = Arc::new(Database::try_new(&cfg.database.url).await.map_err(|e| {
            error!("{}", e.to_string());
            Error::Anyhow(e.into())
        })?);

        let ServiceImpls {
            user_service,
            set_service,
            qna_service,
            // quiz_service,
            // quiz_question_service,
            // test_service,
        } = init_service_implements(db).await;

        Ok(Self {
            user_service,
            set_service,
            qna_service,
            // quiz_service,
            // quiz_question_service,
            // test_service,
        })
    }
}

pub async fn run_server(cfg: Config) -> Result<()> {
    let state = AppState::init(cfg.clone()).await?;

    let app = setup_routing(state);
    let listener = TcpListener::bind(format!("{}:{}", cfg.http.host, cfg.http.port))
        .await
        .map_err(|e| {
            error!("{}", e.to_string());
            Error::Anyhow(e.into())
        })?;

    info!("Server running: {}:{}", cfg.http.host, cfg.http.port);
    axum::serve(listener, app).await.map_err(|e| {
        error!("{}", e.to_string());
        Error::Anyhow(e.into())
    })?;

    Ok(())
}
