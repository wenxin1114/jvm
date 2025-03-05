use std::env;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;
use crate::errors::{Result, JvmError};

pub fn current_version() -> Result<String> {
    let current_version_dir = env::var("JAVA_HOME")
       .map_err(|_| JvmError::Unknown("JAVA_HOME environment variable is not set".to_string()))?;
    
    let path = PathBuf::from(current_version_dir);
    
    let current_version = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();
    Ok(current_version)
}

pub fn list_java_versions() -> Result<()> {
    let jvm_java_home = env::var("JVM_JAVA_HOME")
        .map_err(|_| JvmError::Unknown("JVM_JAVA_HOME environment variable is not set".to_string()))?;
    let java_dir = PathBuf::from(jvm_java_home);
    if !java_dir.exists() {
        return Err(format!("Java directory {} not found", java_dir.display()).as_str().into());
    }

    let mut versions = Vec::new();
    for entry in java_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let version = path.file_name().unwrap().to_str().unwrap().to_string();
            println!("Available version: {}", version);  // 添加这行来显示每个可用版本
            versions.push(version);
        }
    }

    // 获取并显示当前版本
    match current_version() {
        Ok(version) => println!("Current Java version: {}", version),
        Err(_) => println!("Current version: Not set"),
    }
    Ok(())
}

pub fn switch_java_version(version: &str) -> Result<()> {
    let jvm_java_home = env::var("JVM_JAVA_HOME")
        .map_err(|_| JvmError::Unknown("JVM_JAVA_HOME environment variable is not set".to_string()))?;
    let java_dir = PathBuf::from(jvm_java_home).join(version);

    if !java_dir.exists() {
        return Err(format!("Java version {} not found", version).as_str().into());
    }

    // 打开用户环境变量注册表键
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (environment, _) = hkcu.create_subkey(r"Environment")?;

    // 设置 JAVA_HOME
    environment.set_value("JAVA_HOME", &java_dir.to_str().unwrap())?;

    // 更新 PATH
    let current_path: String = environment.get_value("Path")?;
    let bin_path = java_dir.join("bin").to_str().unwrap().to_string();
    
    // 移除旧的 Java bin 路径（如果存在）
    let paths: Vec<&str> = current_path.split(';').collect();
    let new_paths: Vec<&str> = paths
        .iter()
        .filter(|&p| !p.to_lowercase().contains("java") && 
               !p.to_lowercase().contains("jdk") && 
               !p.to_lowercase().contains("jre"))
        .filter(|&p| !p.is_empty())
        .copied()
        .collect();
    
    // 添加新的 Java bin 路径
    let mut new_path = new_paths.join(";");
    if !new_path.is_empty() {
        new_path = format!("{};{}", bin_path, new_path);
    } else {
        new_path = bin_path;
    }
    
    // 删除并重新设置 PATH
    let _ = environment.delete_value("PATH");  // 尝试删除大写版本
    let _ = environment.delete_value("Path");  // 尝试删除首字母大写版本
    let _ = environment.delete_value("path");  // 尝试删除小写版本
    environment.set_value("Path", &new_path)?;

    // 广播环境变量更改消息
    unsafe {
        winapi::um::winuser::SendMessageTimeoutW(
            winapi::um::winuser::HWND_BROADCAST,
            winapi::um::winuser::WM_SETTINGCHANGE,
            0,
            "Environment\0".as_ptr() as winapi::shared::minwindef::LPARAM,
            winapi::um::winuser::SMTO_ABORTIFHUNG,
            5000,
            std::ptr::null_mut(),
        );
    }

    Ok(())
}