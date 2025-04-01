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
    sea_orm::{
        ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set, TransactionTrait,
        TryIntoModel,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizQuestionRepository {
    db: Arc<Database>,
}

impl QuizQuestionRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // pub async fn create_one(
    //     &self,
    //     quiz_id: Uuid,
    //     payload: CreateQuizQuestionRequest,
    // ) -> Result<quiz_questions::Model> {
    //     let conn = self.db.get_connection().await;

    //     let CreateQuizQuestionRequest {
    //         question_content,
    //         answers: _,
    //         r#type,
    //         sample_id,
    //         point,
    //         explaination,
    //         index,
    //     } = payload;

    //     // check not exceed type in question_counts in quiz
    //     // TODO

    //     let return_question = quiz_questions::ActiveModel {
    //         quiz_id: Set(quiz_id),
    //         question_content: Set(question_content),
    //         r#type: Set(r#type),
    //         sample_id: Set(sample_id),
    //         point: Set(point),
    //         explanation: Set(explaination),
    //         index: Set(index),
    //         ..Default::default()
    //     }
    //     .insert(&conn)
    //     .await
    //     .map_err(Error::InsertFailed)?;

    //     Ok(return_question)
    // }

    // pub async fn create_answers(
    //     &self,
    //     question_id: Uuid,
    //     answers: Vec<QuizQuestionAnswer>,
    // ) -> Result<Vec<quiz_question_answers::Model>> {
    //     let conn = self.db.get_connection().await;

    //     let answer_active_models = answers
    //         .into_iter()
    //         .map(|a| {
    //             quiz_question_answers::ActiveModel {
    //                 quiz_question_id: Set(question_id),
    //                 content: Set(a.content),
    //                 is_answer: Set(a.is_answer),
    //                 ..Default::default()
    //             }
    //         })
    //         .collect::<Vec<quiz_question_answers::ActiveModel>>();

    //     QuizQuestionAnswers::insert_many(answer_active_models)
    //         .exec_with_returning_many(&conn)
    //         .await
    //         .map_err(Error::InsertFailed)
    // }

    pub async fn create_many(
        &self,
        quiz_id: Uuid,
        payloads: Vec<CreateQuizQuestionRequest>,
    ) -> Result<Vec<(quiz_questions::Model, Vec<quiz_question_answers::Model>)>> {
        let txn = self
            .db
            .get_connection()
            .await
            .begin()
            .await
            .map_err(|e| Error::BeginTransactionFailed(e))?;

        // Create all answers and associate them with questions
        let mut result = Vec::new();

        // Tạo tất cả questions trước
        let question_models = QuizQuestions::insert_many(payloads.iter().map(|p| {
            quiz_questions::ActiveModel {
                quiz_id: Set(quiz_id),
                sample_id: Set(p.sample_id),
                question_content: Set(p.question_content.clone()),
                r#type: Set(p.r#type.clone()),
                index: Set(p.index),
                point: Set(p.point),
                explanation: Set(p.explaination.clone()),
                ..Default::default()
            }
        }))
        .exec_with_returning_many(&txn)
        .await
        .map_err(Error::InsertFailed)?;

        for (i, question) in question_models.into_iter().enumerate() {
            let answer_active_models = payloads[i]
                .answers
                .iter()
                .map(|a| {
                    quiz_question_answers::ActiveModel {
                        quiz_question_id: Set(question.id),
                        content: Set(a.content.clone()),
                        is_answer: Set(a.is_answer),
                        ..Default::default()
                    }
                })
                .collect::<Vec<_>>();

            let created_answers = QuizQuestionAnswers::insert_many(answer_active_models)
                .exec_with_returning_many(&txn)
                .await
                .map_err(Error::InsertFailed)?;

            result.push((question, created_answers));
        }

        txn.commit()
            .await
            .map_err(|e| Error::CommitTransactionFailed(e))?;

        Ok(result)
    }

    // pub async fn update_one(
    //     &self,
    //     quiz_question_id: Uuid,
    //     quiz_id: Uuid,
    //     payload: UpdateQuizQuestionContent,
    // ) -> Result<Option<(quiz_questions::Model,
    // Vec<quiz_question_answers::Model>)>> {     let conn =
    // self.db.get_connection().await;

    //     let mut quiz_question: quiz_questions::ActiveModel =
    //         QuizQuestions::find_by_id(quiz_question_id)
    //             .filter(
    //                 Condition::all()
    //                     .add(quiz_questions::Column::IsDeleted.eq(false))
    //                     .add(quiz_questions::Column::QuizId.eq(quiz_id)),
    //             )
    //             .one(&conn)
    //             .await
    //             .map_err(Error::QueryFailed)?
    //             .ok_or(Error::RecordNotFound)?
    //             .into();

    //     let UpdateQuizQuestionContent {
    //         question_content,
    //         answers,
    //         r#type,
    //         point,
    //         index: _,
    //     } = payload;

    //     let mut updated = false;
    //     // if question_content is some => Set question_content
    //     if let Some(question_content) = question_content {
    //         quiz_question.question_content = Set(question_content);
    //         updated = true;
    //     }
    //     let updated_answers = if let Some(answers) = answers {
    //         // create transactions for updating answers
    //         let txn = conn.begin().await.map_err(|e| Error::Anyhow(e.into()))?;

    //         for answer in answers.into_iter() {
    //             let mut active_model =
    // quiz_question_answers::ActiveModel::from_json(
    // serde_json::to_value(answer.clone()).map_err(|e| Error::Anyhow(e.into()))?,
    //             )
    //             .map_err(|e| Error::Anyhow(e.into()))?;
    //             if let Some(quiz_question_id) = answer.quiz_question_id {
    //                 active_model.quiz_question_id = Set(quiz_question_id);
    //             }

    //             active_model
    //                 .update(&txn)
    //                 .await
    //                 .map_err(Error::UpdateFailed)?;
    //         }

    //         txn.rollback().await.map_err(Error::UpdateFailed)?;

    //         updated = true;

    //         QuizQuestionAnswers::find()
    //
    // .filter(quiz_question_answers::Column::QuizQuestionId.eq(quiz_question_id))
    //             .filter(quiz_question_answers::Column::IsDeleted.eq(false))
    //             .all(&conn)
    //             .await
    //             .map_err(Error::QueryFailed)?
    //     } else {
    //         vec![]
    //     };
    //     if let Some(r#type) = r#type {
    //         quiz_question.r#type = Set(r#type);
    //         updated = true;
    //     }
    //     if let Some(point) = point {
    //         quiz_question.point = Set(point);
    //         updated = true;
    //     }

    //     if updated {
    //         Ok(Some((
    //             quiz_question
    //                 .update(&conn)
    //                 .await
    //                 .map_err(Error::UpdateFailed)?,
    //             updated_answers,
    //         )))
    //     } else {
    //         Ok(None)
    //     }
    // }

    pub async fn update_many(
        &self,
        payloads: Vec<UpdateQuizQuestionRequest>,
    ) -> Result<Vec<(quiz_questions::Model, Vec<quiz_question_answers::Model>)>> {
        let txn = self
            .db
            .get_connection()
            .await
            .begin()
            .await
            .map_err(Error::BeginTransactionFailed)?;

        let mut result = Vec::new();

        for payload in payloads.into_iter() {
            let mut quiz_question_am = quiz_questions::ActiveModel::from_json(
                serde_json::to_value(&payload.content).map_err(|e| Error::Anyhow(e.into()))?,
            )
            .map_err(|e| Error::Anyhow(e.into()))?;
            quiz_question_am.id = Set(payload.question_id);

            let updated_question = quiz_question_am
                .save(&txn)
                .await
                .map_err(Error::UpdateFailed)?
                .try_into_model()
                .map_err(|e| Error::Anyhow(e.into()))?;

            let mut updated_answers = Vec::new();
            if let Some(answers) = payload.content.answers {
                for answer in answers.into_iter() {
                    let answer_am = quiz_question_answers::ActiveModel::from_json(
                        serde_json::to_value(answer).map_err(|e| Error::Anyhow(e.into()))?,
                    )
                    .map_err(|e| Error::Anyhow(e.into()))?;

                    // Save and convert to Model
                    let updated_answer: quiz_question_answers::Model = answer_am
                        .save(&txn)
                        .await
                        .map_err(Error::UpdateFailed)?
                        .try_into_model()
                        .map_err(|e| Error::Anyhow(e.into()))?;

                    updated_answers.push(updated_answer);
                }
            }

            result.push((updated_question, updated_answers));
        }

        Ok(result)
    }

    pub async fn get_by_id(
        &self,
        quiz_question_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(quiz_questions::Model, Vec<quiz_question_answers::Model>)> {
        let conn = self.db.get_connection().await;

        let quiz_question = QuizQuestions::find_by_id(quiz_question_id)
            .filter(
                Condition::all()
                    .add(quiz_questions::Column::QuizId.eq(quiz_id))
                    .add(quiz_questions::Column::IsDeleted.eq(false)),
            )
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        let quiz_question_answers = QuizQuestionAnswers::find()
            .filter(quiz_question_answers::Column::QuizQuestionId.eq(quiz_question.id))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok((quiz_question, quiz_question_answers))
    }

    pub async fn get_all(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<(quiz_questions::Model, Vec<quiz_question_answers::Model>)>> {
        let conn = self.db.get_connection().await;

        QuizQuestions::find()
            .filter(
                Condition::all()
                    .add(quiz_questions::Column::QuizId.eq(quiz_id))
                    .add(quiz_questions::Column::IsDeleted.eq(false)),
            )
            .find_with_related(QuizQuestionAnswers)
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    pub async fn delete(&self, id: Uuid, quiz_id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;

        let mut quiz_question: quiz_questions::ActiveModel = QuizQuestions::find_by_id(id)
            .filter(
                Condition::all()
                    .add(quiz_questions::Column::QuizId.eq(quiz_id))
                    .add(quiz_questions::Column::IsDeleted.eq(false)),
            )
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

    pub async fn is_of_question(
        &self,
        question_id: Uuid,
        checking_answer_ids: Vec<Uuid>,
    ) -> Result<bool> {
        let conn = self.db.get_connection().await;
        let answers = QuizQuestionAnswers::find()
            .filter(quiz_question_answers::Column::QuizQuestionId.eq(question_id))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        for answer in answers.into_iter() {
            if !checking_answer_ids.contains(&answer.id) {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
