use {
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionCounts {
    pub multiple_choices: u64,
    pub check_boxes: u64,
    pub text_fill: u64,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizRequest {
    pub created_from: Uuid,
    pub is_public: bool,
    pub question_counts: QuestionCounts,
    // pub share_with: Vec<Uuid>, // participants
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizRequest {
    pub is_public: Option<bool>,
    pub publish: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FilterQuizParams {
    pub set_id: Option<Uuid>,
    pub creator_id: Option<Uuid>,
    pub sort_direction: Option<String>,
}
