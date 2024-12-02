use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

use reqwest::header::InvalidHeaderValue as ReqwestInvalidHeader;
use reqwest::Error as ReqwestError;

use std::{env::VarError, fmt};

#[derive(Debug)]
pub enum DownloadError {
    MissingSession,
    InvalidHeaderValue(ReqwestInvalidHeader),
    RequestError(ReqwestError),
}

impl From<VarError> for DownloadError {
    fn from(_value: VarError) -> Self {
        DownloadError::MissingSession
    }
}

impl From<ReqwestInvalidHeader> for DownloadError {
    fn from(value: ReqwestInvalidHeader) -> Self {
        DownloadError::InvalidHeaderValue(value)
    }
}

impl From<ReqwestError> for DownloadError {
    fn from(value: ReqwestError) -> Self {
        DownloadError::RequestError(value)
    }
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSession => {
                write!(f, "Missing env 'SESSION_COOKIE'")
            }
            Self::InvalidHeaderValue(e) => {
                write!(f, "Invalid session header: {}", e)
            }
            Self::RequestError(e) => {
                write!(f, "Error with request: {}", e)
            }
        }
    }
}

impl std::error::Error for DownloadError {}

pub async fn download_input(day: u32) -> Result<String, DownloadError> {
    let session_cookie = std::env::var("SESSION_COOKIE")?;
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    let mut headers = HeaderMap::new();
    let session = format!("session={}", session_cookie);
    headers.insert(COOKIE, HeaderValue::from_str(&session)?);

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    Ok(response.text().await?)
}
