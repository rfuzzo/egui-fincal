use chrono::NaiveDate;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
//#[serde(default)]
pub struct FinItem {
    pub(crate) date: NaiveDate,
    pub(crate) item: String,
    pub(crate) category: String,
    pub(crate) price: f32,
    pub(crate) owner: String,
    pub(crate) ratio: f32,

    // viewmodel
    #[serde(skip)]
    pub(crate) editable: bool,
}
