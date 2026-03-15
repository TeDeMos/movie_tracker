use {
    chrono::NaiveDate,
    reqwest::blocking::Response,
    serde::{
        Deserialize, Deserializer,
        de::{DeserializeOwned, Error},
    },
    serde_json::Value,
    std::{fs, io},
    thiserror::Error,
};

const DEBUG: &str = "/home/tedem/dev/RustroverProjects/movie_tracker/src/temp.json";

pub type ApiResult<T> = Result<T, DebugJsonError>;

#[derive(Error, Debug)]
pub enum DebugJsonError {
    #[error("JSON error: {0}, reference file saved to {DEBUG}")]
    Json(serde_json::Error),
    #[error("Request text error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Other serde error: {0}")]
    SerdeOther(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub trait ResponseExt {
    fn debug_json<T: DeserializeOwned>(self) -> ApiResult<T>;
}

impl ResponseExt for Response {
    fn debug_json<T: DeserializeOwned>(self) -> ApiResult<T> {
        let text = self.text()?;
        let value: Value = serde_json::from_str(&text)?;
        let pretty = serde_json::to_string_pretty(&value)?;
        fs::write(DEBUG, &pretty)?;
        serde_json::from_str(&pretty).map_err(DebugJsonError::Json)
    }
}

pub fn maybe_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where D: Deserializer<'de> {
    match Option::<&str>::deserialize(deserializer)? {
        None | Some("") => Ok(None),
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d").map(Some).map_err(D::Error::custom),
    }
}
