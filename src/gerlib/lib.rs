pub mod accounts;
pub mod changes;
pub mod details;

pub fn get_changes() -> Result<Vec<changes::ChangeInfo>, failure::Error> {
    let json = r#"
[
    {
        "id": "gerrit~master~I488a27893429bfbb0f1aad763806731336858e41",
        "project": "gerrit",
        "branch": "devolop",
        "subject": "RetryHelper#formatCause: Unwrap exceptions wrapped in ExecutionException",
        "status": "DRAFT",
        "updated": "2019-11-20 13:19:54.000000000",
        "_number": 145953
    },
    {
        "id": "gerrit~master~I69fc3684ed0fdd9444a1c2d9b3b211c446c1b2b9",
        "project": "gerrit",
        "branch": "master",
        "subject": "Show reverted changes when clicking \"Revert Submission\"",
        "status": "NEW",
        "updated": "2019-10-05 12:28:07.000000000",
        "_number": 245633
    },
    {
        "id": "gerrit~master~I588a27893429b0bb0f1aad763806731336878ef3",
        "project": "gerrit",
        "branch": "devolop",
        "subject": "Revert \"Unwrap exceptions wrapped in ExecutionException\"",
        "status": "MERGED",
        "updated": "2018-04-01 15:19:00.000000000",
        "_number": 845953
    }
]
"#;

    let changes: Vec<changes::ChangeInfo> = serde_json::from_str(json)
        .map_err(|e| failure::err_msg(format!("JSON parse error: {}\nJSON: {}", e, json)))?;

    Ok(changes)
}
