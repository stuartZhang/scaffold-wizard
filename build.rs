use ::std::{env, fs, os, path::Path, process};
#[cfg(windows)]
fn main() {
    let msys2_home = match env::var("MSYS2_HOME") {
        Ok(value) => value,
        Err(_) => {
            println!("cargo:warning=环境变量`MSYS2_HOME`没有提供，没有链接操作会被执行");
            return;
        }
    };
    println!("调试：MSYS2_HOME={}", msys2_home);
    let out_dir = env::var("OUT_DIR")
        .expect("失败：环境变量`OUT_DIR`未提供");
    println!("调试：OUT_DIR={}", out_dir);
    let exe_dir = Path::new(&out_dir[..]).join("../../..").canonicalize()
        .expect(&format!("失败：不能从 {} 推断出 exe 目录", out_dir)[..]);
    println!("调试：EXE_DIR={}", exe_dir.display());
    if !exe_dir.is_dir() {
        println!("cargo:warning={} 不是一个目录", exe_dir.display());
        process::exit(1);
    }
    let zlib1_exe = exe_dir.join("zlib1.dll");
    println!("调试：ZLIB1_EXE={}", zlib1_exe.display());
    if zlib1_exe.exists() {
        fs::remove_file(zlib1_exe.clone())
            .expect(&format!("失败：不能删除原来的 {} 符号链接文件", zlib1_exe.display())[..]);
    }
    let bits = if cfg!(target_pointer_width = "32") {
        32usize
    } else {
        64usize
    };
    let bin_dir = Path::new(&msys2_home[..]).join(&format!("mingw{}", bits)[..]).join("bin");
    let bin_dir = bin_dir.canonicalize()
        .expect(&format!("失败：不能从 {} 推断出 mingw**/bin 目录", bin_dir.display())[..]);
    println!("调试：BIN_DIR={}", bin_dir.display());
    if !bin_dir.is_dir() {
        println!("cargo:warning={} 不是一个目录", bin_dir.display());
        process::exit(1);
    }
    let zlib1_file = bin_dir.join("zlib1.dll");
    println!("调试：ZLIB1_FILE={}", zlib1_file.display());
    if !zlib1_file.is_file() {
        println!("cargo:warning={} 不是一个文件", zlib1_file.display());
        process::exit(1);
    }
    os::windows::fs::symlink_file(zlib1_file.clone(), zlib1_exe.clone())
        .expect(&format!("失败：不能创建文件链接 {} 指向 {}", zlib1_exe.display(), zlib1_file.display())[..]);
    println!("成功：能创建文件链接 {} 指向 {}", zlib1_exe.display(), zlib1_file.display());
}