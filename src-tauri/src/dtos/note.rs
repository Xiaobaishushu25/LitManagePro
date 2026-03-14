use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

/// 笔记创建请求 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteDto {
    pub document_id: i32,
    pub title: Option<String>,
    pub content: String,
}

/// 笔记更新请求 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteDto {
    pub id: i32,
    pub title: Option<String>,
    pub content: String,
}

/// 笔记响应 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteResponseDto {
    pub id: i32,
    pub document_id: i32,
    pub title: Option<String>,
    pub content: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<crate::entities::prelude::Note> for NoteResponseDto {
    fn from(note: crate::entities::prelude::Note) -> Self {
        Self {
            id: note.id,
            document_id: note.document_id,
            title: note.title,
            content: note.content,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}
