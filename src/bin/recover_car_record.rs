use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn repair_chk_to_avi(chk_file: &Path, output_file: &Path) -> io::Result<()> {
    // 讀取 .CHK 文件內容
    let mut file = File::open(chk_file)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // 檢查 AVI 標頭 (RIFF 和 AVI )
    if let Some(riff_index) = data.windows(4).position(|window| window == b"RIFF") {
        if data[riff_index + 8..riff_index + 12] == *b"AVI " {
            // 提取有效數據
            let valid_data = &data[riff_index..];
            let mut output = File::create(output_file)?;
            output.write_all(valid_data)?;
            println!(
                "成功修復檔案: {} -> {}",
                chk_file.display(),
                output_file.display()
            );
        } else {
            println!("檔案 {} 可能不是有效的 AVI 文件", chk_file.display());
        }
    } else {
        println!(
            "檔案 {} 不包含 RIFF 標頭，可能不是 AVI 文件",
            chk_file.display()
        );
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // 設定 SD 卡路徑
    let sd_card_path = "E:/FOUND.000"; // 修改為 SD 卡目錄
    let output_folder = "repaired_files";

    // 確保輸出目錄存在
    fs::create_dir_all(output_folder)?;

    // 遍歷 SD 卡目錄中的所有 .CHK 檔案
    for entry in fs::read_dir(sd_card_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| ext.to_ascii_lowercase() == "chk")
        {
            let output_file = Path::new(output_folder).join(
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
                    + ".avi",
            );
            if let Err(e) = repair_chk_to_avi(&path, &output_file) {
                eprintln!("修復檔案 {} 時發生錯誤: {}", path.display(), e);
            }
        }
    }

    println!("Finish");

    Ok(())
}
