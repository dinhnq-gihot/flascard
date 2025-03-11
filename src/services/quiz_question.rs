use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{QuizQuestionAnswers, QuizQuestions},
            quiz_question_answers, quiz_questions,
        },
        enums::error::*,
        models::quiz_question::{CreateQuizQuestionRequest, UpdateQuizQuestionRequest},
    },
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait},
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizQuestionService {
    db: Arc<Database>,
}

impl QuizQuestionService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_one(
        &self,
        quiz_id: Uuid,
        payload: CreateQuizQuestionRequest,
    ) -> Result<(quiz_questions::Model, Vec<quiz_question_answers::Model>)> {
        let conn = self.db.get_connection().await;

        let CreateQuizQuestionRequest {
            question_content,
            answers,
            r#type,
            question_id,
        } = payload;

        // check not exceed type in question_counts in quiz
        // TODO

        let return_question = quiz_questions::ActiveModel {
            quiz_id: Set(quiz_id),
            question_id: Set(question_id),
            question_content: Set(question_content),
            r#type: Set(r#type),
            ..Default::default()
        }
        .insert(&conn)
        .await
        .map_err(Error::InsertFailed)?;

        let answer_active_models = answers
            .into_iter()
            .map(|a| {
                quiz_question_answers::ActiveModel {
                    quiz_id: Set(quiz_id),
                    question_id: Set(return_question.id),
                    answer_content: Set(a.content),
                    is_answer: Set(a.is_answer),
                    ..Default::default()
                }
            })
            .collect::<Vec<quiz_question_answers::ActiveModel>>();

        let return_answers = QuizQuestionAnswers::insert_many(answer_active_models)
            .exec_with_returning_many(&conn)
            .await
            .map_err(Error::InsertFailed)?;

        Ok((return_question, return_answers))
    }

    pub async fn update_one(
        &self,
        id: Uuid,
        quiz_id: Uuid,
        payload: UpdateQuizQuestionRequest,
    ) -> Result<Option<quiz_questions::Model>> {
        let conn = self.db.get_connection().await;

        let mut quiz_question: quiz_questions::ActiveModel = QuizQuestions::find_by_id(id)
            .filter(quiz_questions::Column::IsDeleted.eq(false))
            .filter(quiz_questions::Column::QuizId.eq(quiz_id))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let UpdateQuizQuestionRequest {
            question_content,
            answers,
        } = payload;

        let mut updated = false;
        // if question_content is some => Set question_content
        if let Some(question_content) = question_content {
            quiz_question.question_content = Set(question_content);
            updated = true;
        }
        if let Some(answers) = answers {
            let txn = conn.begin().await.map_err(|e| Error::Anyhow(e.into()))?;

            for answer in answers.into_iter() {
                quiz_question_answers::ActiveModel {
                    id: Set(answer.id.unwrap()),
                    answer_content: Set(answer.content),
                    is_answer: Set(answer.is_answer),
                    ..Default::default()
                }
                .update(&txn)
                .await
                .map_err(Error::UpdateFailed)?;
            }

            txn.rollback().await.map_err(Error::UpdateFailed)?;

            updated = true;
        }

        if updated {
            Ok(Some(
                quiz_question
                    .update(&conn)
                    .await
                    .map_err(Error::UpdateFailed)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_id(&self, id: Uuid, quiz_id: Uuid) -> Result<quiz_questions::Model> {
        let conn = self.db.get_connection().await;

        QuizQuestions::find_by_id(id)
            .filter(quiz_questions::Column::IsDeleted.eq(false))
            .filter(quiz_questions::Column::QuizId.eq(quiz_id))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    pub async fn get_all(&self, quiz_id: Uuid) -> Result<Vec<quiz_questions::Model>> {
        let conn = self.db.get_connection().await;

        QuizQuestions::find()
            .filter(quiz_questions::Column::IsDeleted.eq(false))
            .filter(quiz_questions::Column::QuizId.eq(quiz_id))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    pub async fn delete(&self, id: Uuid, quiz_id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;

        let mut quiz_question: quiz_questions::ActiveModel = QuizQuestions::find_by_id(id)
            .filter(quiz_questions::Column::IsDeleted.eq(false))
            .filter(quiz_questions::Column::QuizId.eq(quiz_id))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        quiz_question.is_deleted = Set(true);
        quiz_question
            .update(&conn)
            .await
            .map_err(Error::DeleteFailed)?;

        Ok(())
    }
}
