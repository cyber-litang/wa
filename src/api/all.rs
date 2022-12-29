use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
// {"assignmentId":"1595947592241999873","account":"","current":1,"size":8,"assignmentStatus":null,"sortColumn":"","sortType":null}
pub struct Request {
    assignment_id: String,
    account: String,
    current: i32,
    size: i32,
    assignment_status: Option<i32>,
    sort_column: String,
    sort_type: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            assignment_id: String::new(),
            account: String::new(),
            current: 1,
            size: 300,
            assignment_status: None,
            sort_column: String::new(),
            sort_type: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub code: i32,
    pub success: bool,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub records: Vec<Student>,
}

#[derive(Debug, Deserialize)]
pub struct Student {
    pub account: String,
    pub score: i32,
}

pub async fn all(assignment_id: String) -> Result<Vec<Student>> {
    let client = reqwest::Client::new();
    let req = Request {
        assignment_id,
        ..Default::default()
    };
    let resp = client
        .post("https://apiucloud.bupt.edu.cn/ykt-site/work/all-assignment")
        .json(&req)
        .headers(crate::headers::get_header_map()?)
        .send()
        .await?
        .text()
        .await?;
    let result: Response = serde_json::from_str(&resp)?;
    Ok(result.data.records)
}
