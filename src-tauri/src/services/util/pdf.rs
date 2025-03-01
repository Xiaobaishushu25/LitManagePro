use crate::app_errors::AppResult;
use crate::config::CURRENT_DIR;
use lopdf::Document;
use std::fs;

pub async fn extract_limit_pages(path: &str, id: i32) -> AppResult<String> {
    let new_path = segment_pdf(path, id).await?;
    let content = extract_pdf(&new_path).await?;
    Ok(content)
}
/// 分页pdf
///
async fn segment_pdf(path: &str, id: i32) -> AppResult<String> {
    let mut doc = Document::load(path).unwrap();
    let total_pages = doc.get_pages().keys().len();
    // 如果总页数大于 3，删除第 4 页及以后的页
    if total_pages > 3 {
        let pages_to_delete: Vec<u32> = (4..(total_pages as u32 + 1)).collect();
        doc.delete_pages(&pages_to_delete);
    }
    let tmp_dir = format!("{}/data/tmp", CURRENT_DIR.clone());
    fs::create_dir_all(tmp_dir.clone())?;
    let out_path = format!("{}/output{}.pdf", tmp_dir, id);
    doc.save(out_path.clone())?;
    // println!("前 3 页已成功保留，其他页已删除。输出文件为 output.pdf");
    Ok(out_path)
}
async fn extract_pdf(path: &str) -> AppResult<String> {
    let bytes = std::fs::read(path)?;
    Ok(pdf_extract::extract_text_from_mem(&bytes)?)
}
