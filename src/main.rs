use std::io;

use walkdir::WalkDir;
fn main() {
    local_crate().unwrap();
}


pub(crate) fn local_crate() -> io::Result<()> {
    let path = "/usr/local/cargo/registry/cache"; // 目标目录
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_file() { // 只打印文件
            println!("{}", path.file_name().unwrap().to_string_lossy());
        }
    }
    Ok(())
}