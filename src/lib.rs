use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_char;
use std::{fs::set_permissions, os::unix::fs::PermissionsExt};

use libc::{mount, umount2, EINVAL, MNT_DETACH, MS_BIND, MS_REC};

pub fn lock_value(path: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let mount_path = format!("/data/local/tmp/mount_mask_{}", value);
    unmount(path);

    write_file(&mount_path, value);
    write_file(path, value);
    set_permissions(&mount_path, PermissionsExt::from_mode(0o444))?;
    set_permissions(path, PermissionsExt::from_mode(0o444))?;

    Ok(mount_bind(&mount_path, path)?)
}

fn mount_bind(src_path: &str, dest_path: &str) -> Result<(), String> {
    let src_path = CString::new(src_path).expect("CString::new failed");
    let dest_path = CString::new(dest_path).expect("CString::new failed");

    // 检查目录是否已经被挂载，如果是的，先卸载
    let _ = unsafe { umount2(dest_path.as_ptr(), MNT_DETACH) };

    // 挂载文件系统
    let result = unsafe {
        mount(
            src_path.as_ptr() as *const c_char,
            dest_path.as_ptr() as *const c_char,
            std::ptr::null(),
            MS_BIND | MS_REC,
            std::ptr::null(),
        )
    };

    if result != 0 {
        if result == -EINVAL {
            return Err(String::from("Invalid arguments provided."));
        } else {
            return Err(String::from("Failed to mount filesystem."));
        }
    }

    Ok(())
}

fn unmount(path: &str) {
    let path = CString::new(path).unwrap();
    let _ = unsafe { umount2(path.as_ptr(), MNT_DETACH) };
}

fn write_file(path: &str, content: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    // debug
    // println!("path: {}, value: {}", path, content);

    let _ = set_permissions(path, PermissionsExt::from_mode(0o644));
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
    {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(()) => {}
            Err(e) => eprintln!("Write failed: {}", e),
        },
        Err(e) => eprintln!("Open failed: {}", e),
    }
}
