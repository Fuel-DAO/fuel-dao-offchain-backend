use time::{format_description, OffsetDateTime, UtcOffset};

pub fn format_datetime(timestamp_seconds: u64) -> String {
    let format = format_description::parse("[month repr:short] [day], [year] [hour repr:12]:[minute] [period case:upper]").unwrap();
    // Converting to IST now
    let ist_offset = UtcOffset::from_hms(5, 30, 0).unwrap();
    OffsetDateTime::from_unix_timestamp(timestamp_seconds as i64).unwrap().to_offset(ist_offset).format(&format).unwrap()
}