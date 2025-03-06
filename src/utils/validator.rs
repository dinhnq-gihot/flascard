use crate::{entities::sea_orm_active_enums::QuestionTypeEnum, models::quiz_question::QuizAnswer};

pub fn validate_answer(question_type: &QuestionTypeEnum, answers: &Vec<QuizAnswer>) -> bool {
    match question_type {
        QuestionTypeEnum::CheckBoxes => {
            // MultipleChoice must have exactly one correct answer
            answers.len() > 0 && answers.iter().filter(|a| a.is_correct).count() == 1
        }
        QuestionTypeEnum::MultipleChoice => {
            // Checkboxes must have at least one correct answer
            answers.len() > 0 && answers.iter().any(|a| a.is_correct)
        }
        QuestionTypeEnum::TextFill => {
            // TextFill must have exactly one answer and it must be correct
            answers.len() == 1 && answers[0].is_correct
        }
    }
}
