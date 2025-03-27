use std::collections::HashMap;
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::config::CURRENT_DIR;
use file_icon_provider::get_file_icon;
use image::{DynamicImage, RgbaImage};
use lopdf::Document;
use std::fs;
use std::path::{Path, PathBuf};
use log::{error, info};

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
    // let tmp_dir = format!("{}/data/tmp", CURRENT_DIR.clone());
    let tmp_dir = CURRENT_DIR.join("data").join("tmp");
    fs::create_dir_all(tmp_dir.clone())?;
    clear_tmp_dir(&tmp_dir)?;
    // let out_path = format!("{}/output{}.pdf", tmp_dir, id);
    let out_path = tmp_dir.join(format!("output{}.pdf", id));
    doc.save(out_path.clone())?;
    Ok(out_path.to_string_lossy().to_string())
}
async fn extract_pdf(path: &str) -> AppResult<String> {
    let bytes = std::fs::read(path)?;
    Ok(pdf_extract::extract_text_from_mem(&bytes)?)
}
/// 组织文件到按时间命名的文件夹中，并更新文件路径
///
/// # Arguments
///
/// * `files` - 一个可变的 `HashMap`，键是文件 ID，值是文件的原始路径
///
/// # Returns
///
/// * `AppResult<()>` - 如果操作成功则返回 `Ok(())`，否则返回错误
///
/// # Errors
///
/// * 如果无法获取当前时间、创建文件夹失败、文件不存在或复制文件失败，将返回相应的错误
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use your_crate::organize_files;
///
/// #[tokio::main]
/// async fn main() {
///     let mut files = HashMap::new();
///     files.insert(1, "/path/to/your/file1.txt".to_string());
///     files.insert(2, "/path/to/your/file2.jpg".to_string());
///
///     if let Err(e) = organize_files(&mut files).await {
///         eprintln!("错误: {}", e);
///     } else {
///         println!("所有文件已成功组织");
///         // 打印更新后的文件路径
///         for (id, path) in &files {
///             println!("文件 {}: {}", id, path);
///         }
///     }
/// }
/// ```
pub async fn organize_files(files:&mut HashMap<i32,String>) -> AppResult<()> {
    // 获取当前时间
    let current_date = time::OffsetDateTime::now_utc();
    let time_folder_name = format!("{:04}-{:02}", current_date.year(), current_date.month() as u8);
    // 创建目标文件夹路径
    let target_dir = CURRENT_DIR.join("data").join("files").join(time_folder_name);
    fs::create_dir_all(&target_dir)?;
    // 遍历文件并复制到目标文件夹
    for (id, file_path) in files.iter_mut() {
        let source_path = Path::new(file_path);
        if source_path.exists() && source_path.is_file() {
            // 获取文件名
            if let Some(file_name) = source_path.file_name() {
                let file_name = file_name.to_string_lossy().to_string();
                let dest_path = target_dir.join(&file_name);
                // 复制文件
                fs::copy(source_path, &dest_path)?;
                // 更新 HashMap 中的文件路径为新的路径
                *file_path = dest_path.to_string_lossy().to_string();
                info!("文件 {} 已复制到: {}", id, file_path);
            } else {
                error!("警告: 无法获取文件名: {}", file_path);
            }
        } else {
            error!("警告: 文件不存在或不是有效文件: {}", file_path);
        }
    }
    Ok(())
}
pub fn get_and_save_icon(path: &str, size: u16) -> AppResult<(String, String)> {
    let path = Path::new(path);
    let name = path.file_stem().unwrap().to_str().unwrap();
    let icon = get_file_icon(path, size).map_err(|e| Tip(format!("获取图标失败{:#}", e)))?;
    // let icon_dir = format!("{}/data/icons", CURRENT_DIR.clone());
    let icon_dir = CURRENT_DIR.join("data").join("icons");
    fs::create_dir_all(icon_dir.clone())?;
    let image = RgbaImage::from_raw(icon.width, icon.height, icon.pixels)
        .map(DynamicImage::ImageRgba8)
        .ok_or(Tip("图表格式转换失败".to_string()))?;
    // let save_path = format!("{}/{}.png", icon_dir, name);
    let save_path = icon_dir.join(format!("{}.png", name)).to_string_lossy().to_string();
    image
        .save_with_format(save_path.clone(), image::ImageFormat::Png)
        .map_err(|e| Tip(format!("保存图标失败{:#}", e)))?;
    Ok((name.into(), save_path))
}
fn clear_tmp_dir(tmp_dir: &PathBuf) -> std::io::Result<()> {
    // 检查目录是否存在
    // if !Path::new(tmp_dir).exists() {
    if !tmp_dir.exists() {
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
    use std::collections::HashMap;
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
    #[tokio::test]
    async fn test_organize_files() {
        let mut files = HashMap::new();
        files.insert(1, "F:\\科研\\论文\\I-FGSM.pdf".to_string());
        files.insert(2, "F:\\科研\\论文\\基于频域分析的可迁移对抗攻击算法研究_李腾蛟.pdf".to_string());
        if let Err(e) = super::organize_files(&mut files).await {
            println!("{:#}", e);
        }else { 
            for (id, file_path) in files.iter() {
                println!("文件 {} 的路径为: {}", id, file_path);
            }
        }
    }
}
