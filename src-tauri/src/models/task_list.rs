#[derive(serde::Deserialize, Debug)]
pub struct TaskList {
    pub id: i32,
    pub name: String,
}
