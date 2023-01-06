use chrono::NaiveDate;
use std::{
    fmt::{self},
    str::FromStr,
};

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
//#[serde(default)]
pub struct FinItem {
    pub(crate) date: NaiveDate,
    pub(crate) item: String,
    pub(crate) category: Option<String>,
    pub(crate) price: f32,
    pub(crate) owner: String,
    pub(crate) ratio: f32,

    // viewmodel
    #[serde(skip)]
    pub(crate) editable: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFinItemError;
//pub struct ParseFinItemError(ParseError);

// ToStr as csv
impl fmt::Display for FinItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bind = "None".to_string();
        let cat = self.category.as_ref().unwrap_or(&bind);
        write!(
            f,
            "{},{},{},{},{},{}",
            self.date, self.item, cat, self.price, self.owner, self.ratio
        )
    }
}

// FromStr as csv
impl FromStr for FinItem {
    type Err = ParseFinItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(',').collect();
        if splits.len() != 6 {
            return Err(ParseFinItemError);
        }

        // todo correct format
        let d_fromstr = splits[0]
            .parse::<NaiveDate>()
            .map_err(|_| ParseFinItemError)?;
        let i_fromstr = splits[1].parse::<String>().map_err(|_| ParseFinItemError)?;
        let c_fromstr = splits[2].parse::<String>().map_err(|_| ParseFinItemError)?;
        let p_fromstr = splits[3]
            .trim_end_matches('€')
            .trim_end()
            .parse::<f32>()
            .map_err(|_| ParseFinItemError)?;
        let o_fromstr = splits[4].parse::<String>().map_err(|_| ParseFinItemError)?;
        let r_fromstr = splits[5].parse::<f32>().map_err(|_| ParseFinItemError)?;

        Ok(FinItem {
            date: d_fromstr,
            item: i_fromstr,
            category: Some(c_fromstr),
            price: p_fromstr,
            owner: o_fromstr,
            ratio: r_fromstr,
            // todo: can this be omitted?
            editable: false,
        })
    }
}
