use crate::app_errors::AppResult;
use crate::dtos::note::{CreateNoteDto, NoteResponseDto, UpdateNoteDto};
use crate::entities::prelude::*;
use crate::services::curd::note::NoteCurd;
use crate::services::curd::document::DocumentCurd;
use crate::config::Config;
use log::{error, info};
use sea_orm::ActiveValue::Set;
use sea_orm::sqlx::types::chrono::Utc;
use crate::app_errors::AppError::Tip;
use open;
use std::sync::Mutex;
use tauri::State;

// pub async fn handle_note_command(dto: CreateNoteDto) {
//     info!("处理笔记命令");
//     if dto. {  }
// }

/// 创建笔记
#[tauri::command]
pub async fn create_note(dto: CreateNoteDto) -> Result<NoteResponseDto, String> {
    info!("创建笔记：文档 ID={}", dto.document_id);
    let note = ActiveNote {
        document_id: Set(dto.document_id),
        title: Set(dto.title),
        content: Set(dto.content),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };
    match NoteCurd::create(note).await {
        Ok(created_note) => Ok(NoteResponseDto::from(created_note)),
        Err(e) => {
            error!("创建笔记失败：{}", e);
            Err("创建笔记失败".to_string())
        }
    }
}

/// 获取指定文档的所有笔记
#[tauri::command]
pub async fn get_notes_by_document_id(document_id: i32) -> Result<Vec<NoteResponseDto>, String> {
    info!("获取文档 {} 的所有笔记", document_id);
    match NoteCurd::find_by_document_id(document_id).await {
        Ok(notes) => Ok(notes.into_iter().map(NoteResponseDto::from).collect()),
        Err(e) => {
            error!("获取文档 {} 的所有笔记失败：{}", document_id, e);
            Err("获取笔记失败".to_string())
        }
    }
}

/// 根据 ID 获取笔记
#[tauri::command]
pub async fn get_note_by_id(id: i32) -> Result<Option<NoteResponseDto>, String> {
    info!("获取笔记 {}", id);
    match NoteCurd::find_by_id(id).await {
        Ok(note) => Ok(note.map(NoteResponseDto::from)),
        Err(e) => {
            error!("获取笔记 {} 失败：{}", id, e);
            Err("获取笔记失败".to_string())
        }
    }
}

/// 更新笔记
#[tauri::command]
pub async fn update_note(dto: UpdateNoteDto) -> Result<NoteResponseDto, String> {
    info!("更新笔记 {}", dto.id);
    let existing_note = match NoteCurd::find_by_id(dto.id).await {
        Ok(Some(note)) => note,
        Ok(None) => {
            let err_msg = format!("笔记 {} 不存在", dto.id);
            error!("{}", err_msg);
            return Err(err_msg);
        }
        Err(e) => {
            error!("查找笔记 {} 失败：{}", dto.id, e);
            return Err("查找笔记失败".to_string());
        }
    };
    let note = ActiveNote {
        id: Set(dto.id),
        document_id: Set(existing_note.document_id),
        title: Set(dto.title),
        content: Set(dto.content),
        created_at: Set(existing_note.created_at),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };
    match NoteCurd::update(note).await {
        Ok(updated_note) => Ok(NoteResponseDto::from(updated_note)),
        Err(e) => {
            error!("更新笔记 {} 失败：{}", dto.id, e);
            Err("更新笔记失败".to_string())
        }
    }
}

/// 删除笔记
#[tauri::command]
pub async fn delete_note(id: i32) -> Result<(), String> {
    info!("删除笔记 {}", id);
    match NoteCurd::delete(id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("删除笔记 {} 失败：{}", id, e);
            Err("删除笔记失败".to_string())
        }
    }
}

/// 打开笔记对应的文档
#[tauri::command]
pub async fn open_note_document(config: State<'_, Mutex<Config>>, document_id: i32) -> Result<(), String> {
    info!("打开笔记对应的文档，文档 ID={}", document_id);
    // 获取关联的文档
    let document = match DocumentCurd::find_by_id(document_id).await {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            let err_msg = format!("文档 {} 不存在", document_id);
            error!("{}", err_msg);
            return Err(err_msg);
        }
        Err(e) => {
            error!("查找文档 {} 失败：{}", document_id, e);
            return Err("查找文档失败".to_string());
        }
    };
    
    // 先尝试用天书默认应用打开
    let exes = &config.lock().unwrap().exe_configs;
    if let Some(exe) = exes.iter().find(|exe| exe.is_default) {
        info!("使用天书默认的打开方式：{}", exe.name);
        match open::with(&document.path, exe.path.clone()) {
            Ok(_) => return Ok(()),
            Err(e) => {
                error!("使用天书默认应用打开文档失败：{}", e);
                // 如果天书默认应用打开失败，继续使用系统默认应用
            }
        }
    } else {
        info!("未设置天书默认应用，使用系统默认应用打开");
    }
    // 使用系统默认应用打开文档
    match open::that(&document.path) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("使用系统默认应用打开文档失败：{}", e);
            Err("打开文档失败".to_string())
        }
    }
}

/// 获取所有笔记
#[tauri::command]
pub async fn get_all_notes() -> Result<Vec<NoteResponseDto>, String> {
    info!("获取所有笔记");
    match NoteCurd::find_all().await {
        Ok(notes) => Ok(notes.into_iter().map(NoteResponseDto::from).collect()),
        Err(e) => {
            error!("获取所有笔记失败：{}", e);
            Err("获取所有笔记失败".to_string())
        }
    }
}
