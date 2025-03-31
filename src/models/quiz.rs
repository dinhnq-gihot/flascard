use {
    crate::entities::quizes,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QuestionCounts {
    pub multiple_choices: u64,
    pub check_boxes: u64,
    pub text_fill: u64,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizRequest {
    pub name: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizRequest {
    pub name: Option<String>,
    pub is_public: Option<bool>,
    pub is_publish: Option<bool>,
    pub question_counts: Option<QuestionCounts>,
    pub total_point: Option<i32>,
}

// #[derive(Debug, Deserialize)]
// pub struct QuizFilterVisibilities {
//     pub public: Option<()>,
//     pub publish: Option<()>,
//     pub owned: Option<()>,
//     pub shared: Option<()>,
// }

#[derive(Debug, Deserialize)]
pub struct FilterQuizParams {
    pub name: Option<String>,
    pub creator_id: Option<Uuid>,
    pub visibility: Option<Vec<String>>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct QuizWithVisibility {
    pub quiz: quizes::Model,
    pub visibility: Vec<String>,
}
