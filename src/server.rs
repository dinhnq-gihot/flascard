use {
    crate::{
        config::Config,
        db::db_connection::Database,
        enums::error::{Error, Result},
        error,
        routes::setup_routing,
        services::{
            prelude::*, quiz::QuizService, quiz_question::QuizQuestionService,
            shared_quiz::SharedQuizService, test::TestService,
        },
    },
    std::sync::Arc,
    tokio::net::TcpListener,
    tracing::info,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub set_service: Arc<SetService>,
    pub shared_set_service: Arc<SharedSetService>,
    pub qna_service: Arc<QnAService>,
    pub quiz_service: Arc<QuizService>,
    pub quiz_question_service: Arc<QuizQuestionService>,
    pub shared_quiz_service: Arc<SharedQuizService>,
    pub test_service: Arc<TestService>,
}

impl AppState {
    pub async fn init(cfg: Config) -> Result<Self> {
        let db = Arc::new(Database::try_new(&cfg.database.url).await.map_err(|e| {
            error!("{}", e.to_string());
            Error::Anyhow(e.into())
        })?);

        Ok(Self {
            user_service: Arc::new(UserService::new(Arc::clone(&db))),
            set_service: Arc::new(SetService::new(Arc::clone(&db))),
            shared_set_service: Arc::new(SharedSetService::new(Arc::clone(&db))),
            qna_service: Arc::new(QnAService::new(Arc::clone(&db))),
            quiz_service: Arc::new(QuizService::new(Arc::clone(&db))),
            quiz_question_service: Arc::new(QuizQuestionService::new(Arc::clone(&db))),
            shared_quiz_service: Arc::new(SharedQuizService::new(Arc::clone(&db))),
            test_service: Arc::new(TestService::new(Arc::clone(&db))),
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
