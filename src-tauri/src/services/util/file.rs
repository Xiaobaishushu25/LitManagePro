use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::config::CURRENT_DIR;
use file_icon_provider::get_file_icon;
use image::{DynamicImage, RgbaImage};
use lopdf::Document;
use std::fs;
use std::path::Path;
pub async fn extract_limit_pages(path: &str, id: i32) -> AppResult<String> {
    let new_path = segment_pdf(path, id).await?;
    let content = extract_pdf(&new_path).await?;
    Ok(content)
}
/// 分页pdf
///
async fn segment_pdf(path: &str, id: i32) -> AppResult<String> {
    let mut doc = Document::load(path).map_err(|e| Tip(format!("加载pdf失败{:#}", e)))?;
    let total_pages = doc.get_pages().keys().len();
    // 如果总页数大于 3，删除第 4 页及以后的页
    if total_pages > 3 {
        let pages_to_delete: Vec<u32> = (4..(total_pages as u32 + 1)).collect();
        doc.delete_pages(&pages_to_delete);
    }
    let tmp_dir = format!("{}/data/tmp", CURRENT_DIR.clone());
    fs::create_dir_all(tmp_dir.clone())?;
    clear_tmp_dir(&tmp_dir)?;
    let out_path = format!("{}/output{}.pdf", tmp_dir, id);
    doc.save(out_path.clone())?;
    // println!("前 3 页已成功保留，其他页已删除。输出文件为 output.pdf");
    Ok(out_path)
}
async fn extract_pdf(path: &str) -> AppResult<String> {
    let bytes = std::fs::read(path)?;
    Ok(pdf_extract::extract_text_from_mem(&bytes)?)
}

pub fn get_and_save_icon(path: &str, size: u16) -> AppResult<(String, String)> {
    let path = Path::new(path);
    let name = path.file_stem().unwrap().to_str().unwrap();
    let icon = get_file_icon(path, size).map_err(|e| Tip(format!("获取图标失败{:#}", e)))?;
    let icon_dir = format!("{}/data/icons", CURRENT_DIR.clone());
    fs::create_dir_all(icon_dir.clone())?;
    let image = RgbaImage::from_raw(icon.width, icon.height, icon.pixels)
        .map(DynamicImage::ImageRgba8)
        .ok_or(Tip("图表格式转换失败".to_string()))?;
    let save_path = format!("{}/{}.png", icon_dir, name);
    image
        .save_with_format(save_path.clone(), image::ImageFormat::Png)
        .map_err(|e| Tip(format!("保存图标失败{:#}", e)))?;
    Ok((name.into(), save_path))
}
fn clear_tmp_dir(tmp_dir: &str) -> std::io::Result<()> {
    // 检查目录是否存在
    if !Path::new(tmp_dir).exists() {
        return Ok(()); // 如果目录不存在，直接返回成功
    }
    // 获取目录中的所有条目
    let entries = fs::read_dir(tmp_dir)?;
    // 遍历每个条目并删除
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        // 如果是文件，直接删除
        if path.is_file() {
            fs::remove_file(&path)?;
        }
        // 如果是目录，递归删除目录及其内容
        else if path.is_dir() {
            fs::remove_dir_all(&path)?;
        }
    }
    Ok(())
}
#[cfg(test)]
mod test {
    use std::fs::File;

    #[test]
    fn test_open_pdf() {
        // todo lopdf无法加载一些文件
        // let path = "F:\\科研\\论文\\基于对抗样本的神经网络安全性问题研究综述_李紫珊.pdf";
        let path = "F:\\科研\\论文\\基于频域分析的可迁移对抗攻击算法研究_李腾蛟.pdf";
        // let path = "F:\\科研\\论文\\RTRGAN-崔宝杰.pdf";
        if let Err(e) = lopdf::Document::load(path) {
            println!("{:#}", e);
        }
        let file = File::open(path).unwrap();
        let metadata = file.metadata().unwrap();
        println!("{:?}", metadata);
    }
}
