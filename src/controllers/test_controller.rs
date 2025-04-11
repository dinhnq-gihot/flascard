use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        models::test::{CreateTest, QueryTestParams, ResolveTestRequest},
        server::AppState,
        utils::jwt::Claims,
    },
    axum::{
        extract::{Path, Query, State},
        response::IntoResponse,
        Extension, Json,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct TestController;

impl TestController {
    pub async fn get_all(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Query(params): Query<QueryTestParams>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service.get_all(caller.id, params).await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(test_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service.get_by_id(caller.id, test_id).await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<CreateTest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service.create_one(caller.id, payload).await?;

        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn start(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(test_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service.start_one(caller.id, test_id).await?;

        Ok(into_ok_response("Started successfully".into(), Some(res)))
    }

    pub async fn get_testing_question(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(test_id): Path<Uuid>,
        Path(quiz_question_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service
            .get_testing_question(caller.id, test_id, quiz_question_id)
            .await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    pub async fn get_all_testing_question_statuses(
        State(state): State<AppState>,
        Extension(_): Extension<Claims>,
        Path(test_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service.get_all_testing_question_results(test_id).await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    // Khi ấn next/previous/chọn câu bất kỳ/thoát => chương trình lưu lại câu trả
    // lời vào test_result và ghi lại câu quiz đang làm vào test_state
    pub async fn resolve_test_question(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(test_id): Path<Uuid>,
        Path(question_id): Path<Uuid>,
        Json(payload): Json<ResolveTestRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service
            .resolve_testing_question(caller.id, test_id, question_id, payload)
            .await?;

        Ok(into_ok_response("Resolved successfully".into(), res))
    }

    // Nhấn nút submit => chương trình chấm điểm các test result
    pub async fn submit(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(test_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service.submit_one(caller.id, test_id).await?;

        Ok(into_ok_response("Submitted successfully".into(), Some(res)))
    }

    // pub async fn save_state(State(state): State<AppState>) -> Result<impl
    // IntoResponse> {     Ok(())
    // }

    pub async fn result(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(test_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service.result(caller.id, test_id).await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    pub async fn review_solution(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(test_id): Path<Uuid>,
        Path(quiz_question_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let res = service
            .review_solution(caller.id, test_id, quiz_question_id)
            .await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }
}
