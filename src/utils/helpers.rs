use chrono::NaiveDateTime;

pub fn check_test_status(
    started_at: Option<NaiveDateTime>,
    submitted_at: Option<NaiveDateTime>,
) -> String {
    if started_at.is_none() && submitted_at.is_none() {
        "Not Started".into()
    } else if started_at.is_some() && submitted_at.is_none() {
        "In Progess".into()
    } else if submitted_at.is_some() {
        "Completed".into()
    } else {
        "".into()
    }
}
