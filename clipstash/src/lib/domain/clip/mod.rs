use serde:: { Deserialize, Serialize };
use thiserror::Error;
use uuid::Error;
use std::num::TryFromIntError;


pub mod field;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("no content")]
    EmptyContent,
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),

}

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct Clip {

    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
    

    

    


}