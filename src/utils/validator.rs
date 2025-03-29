use crate::{
    entities::sea_orm_active_enums::QuestionTypeEnum, models::quiz_question::QuizQuestionAnswer,
};

pub fn validate_answer(question_type: &QuestionTypeEnum, answers: &[QuizQuestionAnswer]) -> bool {
    match question_type {
        QuestionTypeEnum::CheckBoxes => {
            // CheckBoxes must have exactly one correct answer
            !answers.is_empty() && answers.iter().filter(|a| a.is_answer).count() == 1
        }
        QuestionTypeEnum::MultipleChoice => {
            // MultipleChoice must have at least one correct answer
            !answers.is_empty() && answers.iter().any(|a| a.is_answer)
        }
        QuestionTypeEnum::TextFill => {
            // TextFill must have exactly one answer and it must be correct
            answers.len() == 1 && answers[0].is_answer
        }
    }
}

pub fn all_quiz_answers_contain_id(answers: &[QuizQuestionAnswer]) -> bool {
    for a in answers.iter() {
        if a.id.is_none() {
            return false;
        }
    }
    true
}
