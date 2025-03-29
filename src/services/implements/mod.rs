use {
    super::{implements::prelude::*, traits::prelude::*},
    crate::{
        db::db_connection::Database,
        repositories::{
            question::QnARepository, //quiz::QuizRepository,
            // quiz_question::QuizQuestionRepository,
            set::SetRepository,
            user::UserRepository, //test::TestRepository, ,
        },
    },
    std::sync::Arc,
};

pub mod prelude;
pub mod qna_impl;
// pub mod quiz_impl;
// pub mod quiz_question_impl;
pub mod set_impl;
// pub mod test_impl;
pub mod user_impl;

pub struct ServiceImpls {
    pub user_service: Arc<dyn UserService>,
    pub set_service: Arc<dyn SetService>,
    pub qna_service: Arc<dyn QnAService>,
    // pub quiz_service: Arc<dyn QuizService>,
    // pub quiz_question_service: Arc<dyn QuizQuestionService>,
    // pub test_service: Arc<dyn TestService>,
}

pub async fn init_service_implements(db: Arc<Database>) -> ServiceImpls {
    let user_service = Arc::new(UserServiceImpl::new(Arc::new(UserRepository::new(
        Arc::clone(&db),
    ))));
    let set_service = Arc::new(SetServiceImpl::new(Arc::new(SetRepository::new(
        Arc::clone(&db),
    ))));
    let qna_service = Arc::new(QnAServiceImpl::new(
        Arc::new(QnARepository::new(Arc::clone(&db))),
        set_service.clone(),
    ));
    // let quiz_service =
    // Arc::new(QuizServiceImpl::new(Arc::new(QuizRepository::new(
    //     Arc::clone(&db),
    // ))));
    // let quiz_question_service = Arc::new(QuizQuestionServiceImpl::new(
    //     Arc::new(QuizQuestionRepository::new(Arc::clone(&db))),
    //     quiz_service.clone(),
    // ));
    // let test_service = Arc::new(TestServiceImpl::new(
    //     Arc::new(TestRepository::new(Arc::clone(&db))),
    //     quiz_service.clone(),
    //     quiz_question_service.clone(),
    //     set_service.clone(),
    // ));

    ServiceImpls {
        user_service,
        set_service,
        qna_service,
        // quiz_service,
        // quiz_question_service,
        // test_service,
    }
}
