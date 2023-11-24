#[derive(serde::Serialize)]
pub struct Response {
    pub code: u16,
    pub msg: String,
    pub data: String,
}
