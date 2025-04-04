pub use sea_orm_migration::prelude::*;

mod m20250223_061404_create_users_table;
mod m20250223_064318_create_sets_table;
mod m20250223_065024_create_questions_table;
// mod m20250223_065613_create_answers_table;
mod m20250223_070735_create_quizes_table;
mod m20250223_071935_create_quiz_questions_table;
mod m20250223_072007_create_quiz_question_anwsers_table;
mod m20250223_072118_create_shared_quizes_table;
mod m20250223_072123_create_shared_sets_table;
mod m20250223_075910_create_tests_table;
// mod m20250223_075918_create_test_states_table;
mod m20250223_075930_create_test_answers_table;
mod m20250404_031734_create_test_question_results_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250223_061404_create_users_table::Migration),
            Box::new(m20250223_064318_create_sets_table::Migration),
            Box::new(m20250223_065024_create_questions_table::Migration),
            Box::new(m20250223_070735_create_quizes_table::Migration),
            Box::new(m20250223_071935_create_quiz_questions_table::Migration),
            Box::new(m20250223_072007_create_quiz_question_anwsers_table::Migration),
            Box::new(m20250223_072118_create_shared_quizes_table::Migration),
            Box::new(m20250223_072123_create_shared_sets_table::Migration),
            Box::new(m20250223_075910_create_tests_table::Migration),
            Box::new(m20250223_075930_create_test_answers_table::Migration),
            Box::new(m20250404_031734_create_test_question_results_table::Migration),
        ]
    }
}
