use chrono::NaiveDate;
use std::{fmt, str::FromStr};

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFinItemError;

// ToStr as csv
impl fmt::Display for FinItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{}",
            self.date, self.item, self.category, self.price, self.owner, self.ratio
        )
    }
}

// FromStr as csv
impl FromStr for FinItem {
    type Err = ParseFinItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(';').collect();
        if splits.len() != 6 {
            return Err(ParseFinItemError);
        }

        let d_fromstr = splits[0]
            .parse::<NaiveDate>()
            .map_err(|_| ParseFinItemError)?;
        let i_fromstr = splits[0].parse::<String>().map_err(|_| ParseFinItemError)?;
        let c_fromstr = splits[0].parse::<String>().map_err(|_| ParseFinItemError)?;
        let p_fromstr = splits[0].parse::<f32>().map_err(|_| ParseFinItemError)?;
        let o_fromstr = splits[0].parse::<String>().map_err(|_| ParseFinItemError)?;
        let r_fromstr = splits[0].parse::<f32>().map_err(|_| ParseFinItemError)?;

        Ok(FinItem {
            date: d_fromstr,
            item: i_fromstr,
            category: c_fromstr,
            price: p_fromstr,
            owner: o_fromstr,
            ratio: r_fromstr,
            // todo
            editable: false,
        })
    }
}
