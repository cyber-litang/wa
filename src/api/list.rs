use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
// {"siteId":"1558997828460613635","keyword":"","current":1,"size":64,"status":0,"sortColumn":"","sortType":null}
pub struct Request {
    site_id: String,
    keyword: String,
    current: i32,
    size: i32,
    status: i32,
    sort_column: String,
    sort_type: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            site_id: String::new(),
            keyword: String::new(),
            current: 1,
            size: 64,
            status: 0,
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
    pub records: Vec<Assignment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Assignment {
    pub id: String,
    #[serde(rename = "assignmentTitle")]
    pub title: String,
}

fn map_title(title: String) -> String {
    title
        .split("作业逾期")
        .map(|s| s.to_string())
        .next()
        .unwrap_or(title)
}

pub async fn list(site_id: String) -> Result<Vec<Assignment>> {
    let client = reqwest::Client::new();
    let req = Request {
        site_id,
        ..Default::default()
    };
    let resp = client
        .post("https://apiucloud.bupt.edu.cn/ykt-site/work/teacher/list")
        .json(&req)
        .headers(crate::headers::get_header_map()?)
        .send()
        .await?
        .text()
        .await?;
    let result: Response = serde_json::from_str(&resp)?;

    let records: Vec<_> = result
        .data
        .records
        .clone()
        .into_iter()
        .map(|v| Assignment {
            title: map_title(v.title),
            id: v.id,
        })
        .collect();

    Ok(records)
}
