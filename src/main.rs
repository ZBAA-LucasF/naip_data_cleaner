use std::{fs, io};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use naip_data_cleaner::{get_filepath_from_db, list_db_path};

fn main() -> Result<(), &'static str> {
    println!("请输入数据文件夹的路径：");

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap_or_default();

    let data_path_text = input_text.clone();
    let data_path = Path::new(data_path_text.trim());

    let db_path = list_db_path(data_path)?;

    println!();

    for (key, value) in db_path.iter().enumerate() {
        println!("{}: {}", key, value.display());
    }

    println!();

    input_text = "".to_string();
    println!("请输入要留下的数据库编号，以英文逗号进行分隔:");

    io::stdin().read_line(&mut input_text).unwrap_or_default();

    println!();

    let db_id: HashSet<usize> = input_text
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().expect("请输入正确的内容"))
        .filter(|&x| x < db_path.len())
        .collect();

    let mut valid_file: HashSet<PathBuf> = HashSet::new();

    for i in db_id {
        valid_file.extend(
            get_filepath_from_db(db_path[i].as_path())?
                .iter()
                .map(|x| data_path.join(x)),
        );
        valid_file.insert(db_path[i].clone().to_path_buf());
    }

    let target_dir: Vec<PathBuf> = [
        "Customers",
        "Enroute",
        "ExtraFiles",
        "GeneralDoc",
        "Terminal",
    ]
        .into_iter()
        .map(|x| data_path.join(x))
        .collect();

    let mut remove_files = vec![];

    for dir in target_dir {
        remove_files.extend(
            WalkDir::new(dir)
                .into_iter()
                .filter(|x| x.as_ref().expect("读取文件夹失败").file_type().is_file())
                .filter(|x| !valid_file.contains(x.as_ref().expect("读取文件夹失败").path()))
                .map(|x| x.expect("获取路径失败").into_path()),
        );
    }

    for file in remove_files {
        let file_cloned = file.clone();
        match fs::remove_file(file) {
            Ok(_) => println!("删除 {} 成功", file_cloned.display()),
            _ => println!("!!!!!!!!!! 删除 {} 失败", file_cloned.display()),
        };
    }

    println!("操作完成");

    Ok(())
}
