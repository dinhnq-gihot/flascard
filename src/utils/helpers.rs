use {crate::models::quiz::QuestionCounts, chrono::NaiveDateTime};

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

pub fn total_question_count(counts: QuestionCounts) -> u64 {
    counts.check_boxes + counts.check_boxes + counts.text_fill
}
