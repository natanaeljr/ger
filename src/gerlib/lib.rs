pub mod accounts;
pub mod changes;
pub mod details;

pub fn get_changes() -> Result<Vec<changes::ChangeInfo>, failure::Error> {
    let json = r#"
[
    {
        "id": "gerrit~master~I69fc3684ed0fdd9444a1c2d9b3b211c446c1b2b9",
        "project": "gerrit",
        "branch": "master",
        "subject": "Show reverted changes when clicking \"Revert Submission\"",
        "status": "NEW",
        "updated": "2019-10-05 02:28:07.000000000",
        "_number": 245633
    },
    {
        "id": "gerrit~master~I488a27893429bfbb0f1aad763806731336858e41",
        "project": "gerrit",
        "branch": "devolop",
        "subject": "RetryHelper#formatCause: Unwrap exceptions wrapped in ExecutionException",
        "status": "DRAFT",
        "updated": "2019-11-18 22:19:54.000000000",
        "_number": 145953
    }
]
"#;

    let changes: Vec<changes::ChangeInfo> = serde_json::from_str(json)
        .map_err(|e| failure::err_msg(format!("JSON parse error: {}\nJSON: {}", e, json)))?;

    Ok(changes)
}
