pub mod all;
pub mod list;

use all::Student;
use anyhow::Result;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    time::Duration,
};
use anyhow::{Context};
use std::{iter::repeat, path::Path};

#[derive(Debug, Default)]
pub struct AllScores {
    pub assignments: Vec<String>,
    pub scores: BTreeMap<String, Vec<i32>>,
}

impl AllScores {
    pub fn new(data: Vec<(String, HashMap<String, i32>)>) -> AllScores {
        let data = {
            let mut new_data: BTreeMap<String, HashMap<String, i32>> = BTreeMap::new();
            for (assignment, students) in data {
                new_data.entry(assignment).or_default().extend(students);
            }
            new_data
        };
        let students: BTreeSet<_> = data.iter().flat_map(|(_, v)| v.keys()).cloned().collect();

        let mut assignments = Vec::new();
        let mut scores: BTreeMap<String, Vec<i32>> = BTreeMap::new();

        for (assignment, student_scores) in data {
            assignments.push(assignment.clone());
            for student in &students {
                let score = student_scores.get(student).cloned().unwrap_or(0);
                scores.entry(student.clone()).or_default().push(score);
            }
        }

        AllScores {
            assignments,
            scores,
        }
    }

    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let assignments = {
            let mut assignments = vec!["".to_string()];
            assignments.extend_from_slice(&self.assignments);
            assignments
        };
        let mut writer = csv::Writer::from_path(path)?;
        writer.write_record(assignments)?;
        for (student, scores) in &self.scores {
            let mut record = vec![student.clone()];
            record.extend_from_slice(&scores.iter().map(|v| v.to_string()).collect::<Vec<_>>());
            writer.write_record(record)?;
        }
        Ok(())
    }
}

pub async fn list_all_scores(site_id: String) -> Result<AllScores> {
    let assignments = list::list(site_id).await?;
    let mut result = Vec::new();
    for assignment in assignments {
        let students: HashMap<_, _> = all::all(assignment.id)
            .await?
            .into_iter()
            .map(|v| (v.account, v.score))
            .collect();
        result.push((assignment.title, students));
        tokio::time::sleep(Duration::from_millis(400)).await;
    }
    Ok(AllScores::new(result))
}
