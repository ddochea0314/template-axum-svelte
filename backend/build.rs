use std::{env, fs};
use std::path::Path;

fn main() {

    let profile = env::var("PROFILE");
    if profile.is_err() {
        panic!("PROFILE environment variable is not set");
    }

    let target_path = format!("../target/{}/static", profile.unwrap());
    let target_dir = Path::new(&target_path);
    let src_dir = Path::new("static");

    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).unwrap();
    }

    // Repository 처음 clone 한 경우, npm install 에서 npm build까지 시간이 걸릴 수 있으므로 최대30초 대기
    if src_dir.exists().eq(&false) {
        let mut attempts = 0;
        while attempts < 3 {
            if src_dir.exists() {
            break;
            }
            attempts += 1;
            std::thread::sleep(std::time::Duration::from_secs(10));
        }

        if !src_dir.exists() {
            panic!("src/static directory does not exist. you need to `npm run build` in frontend directory");
        }
    }

    fs::create_dir_all(&target_dir).unwrap();
    copy_dir(src_dir, target_dir).unwrap();
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_dir(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }

    Ok(())
}