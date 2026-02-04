use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::error::Error;
use std::fmt;

/// Video の識別子（newtype）
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoId(Uuid);

impl VideoId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

/// ドメインエラー（必要に応じて拡張）
/// エラー内容を表す列挙型
#[derive(Debug)]
pub enum DomainError {
    InvalidTitle,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidTitle => write!(f, "invalid title"),
        }
    }
}

impl Error for DomainError {}

/// Video エンティティ
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: VideoId,
    pub title: String,
    pub description: Option<String>,
}

impl Video {
    /// コンストラクタ（不変条件チェックをここで行う）
    pub fn new(title: impl Into<String>, description: Option<String>) -> Result<Self, DomainError> {
        let title = title.into();
        let title_trim = title.trim();
        if title_trim.is_empty() || title_trim.len() > 255 {
            return Err(DomainError::InvalidTitle);
        }
        Ok(Self {
            id: VideoId::new(),
            title: title_trim.to_string(),
            description,
        })
    }

    /// 状態変更メソッド（ドメインルールをここに置く）
    pub fn change_title(&mut self, new_title: impl Into<String>) -> Result<(), DomainError> {
        let t = new_title.into();
        let t_trim = t.trim();
        if t_trim.is_empty() || t_trim.len() > 255 {
            return Err(DomainError::InvalidTitle);
        }
        self.title = t_trim.to_string();
        Ok(())
    }
}