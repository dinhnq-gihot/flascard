use {
    axum::{http::StatusCode, Json},
    serde::{Serialize, Serializer},
};

#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct GenericResponse<T> {
    #[serde(serialize_with = "serialize_status_code")]
    pub status: StatusCode,
    pub result: DataResponse<T>,
}

// Custom serializer function for StatusCode
fn serialize_status_code<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&status.to_string()) // Converts StatusCode to
                                                  // String
}

pub fn into_ok_response<T>(msg: String, data: Option<T>) -> (StatusCode, Json<GenericResponse<T>>) {
    let status = StatusCode::OK;
    let body = GenericResponse {
        status,
        result: DataResponse { msg, data },
    };

    (status, Json(body))
}
