use {
    chrono::NaiveDateTime,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateTest {
    pub quiz_id: Uuid,
    pub max_duration: u64,
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
    pub current_question_id: Option<Uuid>,
    pub completed_questions: i32,
    pub spent_time_in_second: i32,
}

#[derive(Debug, Serialize)]
pub struct TestModel {
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
    pub sort_direction: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
