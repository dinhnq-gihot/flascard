use {
    super::quiz_question::QuizQuestionResponse,
    crate::entities::{
        quiz_question_answers,
        sea_orm_active_enums::{QuestionTypeEnum, StatusEnum},
        test_answers,
    },
    chrono::NaiveDateTime,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateTest {
    pub quiz_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateTestResponse {
    pub id: Uuid,
    pub max_duration: i32,
    pub quiz: TestingQuiz,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct TestingQuiz {
    pub id: Uuid,
    pub name: String,
    pub set_id: Uuid,
    pub set_name: String,
}

#[derive(Debug, Serialize)]
pub struct CurrentTestState {
    pub current_question_id: Uuid,
    pub completed_questions: i32,
    pub spent_time_in_second: i32,
}

#[derive(Debug, Serialize)]
pub struct TestResponse {
    pub id: Uuid,
    pub quiz: TestingQuiz,
    pub status: String,
    pub score: Option<i32>,
    pub created_at: NaiveDateTime,
    pub started_at: Option<NaiveDateTime>,
    pub submitted_at: Option<NaiveDateTime>,
    pub max_duration: i32,
    pub remaining_time: i32,
    pub current_state: Option<CurrentTestState>,
}

#[derive(Debug, Deserialize)]
pub struct QueryTestParams {
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u64>,
}

#[derive(Debug, Default)]
pub struct UpdateTest {
    pub started_at: Option<NaiveDateTime>,
    pub submitted_at: Option<NaiveDateTime>,
    pub status: Option<StatusEnum>,
    pub current_testing_quiz_question: Option<Uuid>,
    pub resolved_count: Option<i32>,
    pub remaining_time: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TestingAnswer {
    pub id: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct TestingQuestion {
    pub id: Uuid,
    pub content: String,
    pub r#type: QuestionTypeEnum,
    pub answers: Vec<TestingAnswer>,
    pub user_answers: Vec<test_answers::Model>,
}

#[derive(Debug, Serialize)]
pub struct StartTestResponse {
    pub id: Uuid,
    pub started_at: NaiveDateTime,
    pub current_question: TestingQuestion,
    pub remainning_time: i32,
}

impl From<quiz_question_answers::Model> for TestingAnswer {
    fn from(value: quiz_question_answers::Model) -> Self {
        Self {
            id: value.id,
            content: value.content,
        }
    }
}

impl From<QuizQuestionResponse> for TestingQuestion {
    fn from(value: QuizQuestionResponse) -> Self {
        let QuizQuestionResponse { question, answers } = value;

        Self {
            id: question.id,
            content: question.question_content,
            r#type: question.r#type,
            answers: answers.into_iter().map(Into::into).collect(),
            user_answers: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveTestAnswer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub selected_answer_id: Option<Uuid>,
    pub text_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_time_in_second: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolveTestRequest {
    pub save_test_answers: Vec<SaveTestAnswer>,
    pub remaining_time: i32,
}

#[derive(Debug, Serialize)]
pub struct ResolveResponse {
    pub next_question_id: Option<Uuid>,
}
