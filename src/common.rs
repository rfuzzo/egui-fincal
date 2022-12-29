// use std::{
//     fs::File,
//     io::{self, BufRead},
//     path::Path,
// };

use chrono::Month;
use num_traits::FromPrimitive;

/// Get English name of month for index
pub(crate) fn to_name(month_idx: u32) -> String {
    let some_month = Month::from_u32(month_idx);
    match some_month {
        Some(month) => month.name().to_owned(),
        None => "".to_owned(),
    }
}

////////////////////////////////
////////////// IO //////////////
////////////////////////////////

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
