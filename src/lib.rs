use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, Statement};

/// 列出数据文件夹中的版本数据库路径
pub fn list_db_path(root_path: &Path) -> Result<Vec<PathBuf>, &'static str> {
    let customers_dbs = root_path.join("Customers");

    let db_path: Vec<PathBuf> = fs::read_dir(customers_dbs)
        .expect("读取数据库文件夹失败")
        .map(|x| x.expect("列出数据库文件夹列表失败").path())
        .collect();

    Ok(db_path)
}

/// 读取单个数据库文件
pub fn get_filepath_from_db(db_path: &Path) -> Result<Vec<PathBuf>, &'static str> {
    let conn = Connection::open(db_path).expect("读取数据库失败");

    let mut querys: Vec<Statement> = [
        "SELECT FilePath FROM Charts",
        "SELECT ZipPath FROM EnrouteParts",
        "SELECT FilePath FROM EXTRA_FILES",
        "SELECT Document FROM GENERAL_DOC WHERE Document <> ''",
    ]
        .into_iter()
        .map(|x| conn.prepare(x).unwrap())
        .collect();

    let mut file_paths: Vec<PathBuf> = vec![];

    for i in querys.iter_mut() {
        let temp_path: Vec<PathBuf> = i
            .query_map([], |row| -> rusqlite::Result<String> { row.get(0) })
            .expect("数据库读取错误")
            .map(|x| x.expect("数据库行读取错误"))
            .map(PathBuf::from)
            .collect();

        file_paths.extend(temp_path);
    }

    Ok(file_paths)
}
