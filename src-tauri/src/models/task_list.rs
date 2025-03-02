use serde::Serialize;

#[derive(serde::Deserialize, Debug, Serialize)]
pub struct TaskList {
    pub id: u64,
    pub name: String,
}
