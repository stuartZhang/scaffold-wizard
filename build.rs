use ::std::{env, fs, io::Result as IoResult, os, path::{Path, PathBuf}, process};
use ::cbindgen::{Builder, Config};
const ASSETS_DIR_NAME: &str = "assets";
fn main() {
    let out_dir = env::var("OUT_DIR")
        .expect("失败：环境变量`OUT_DIR`未提供");
    println!("调试：OUT_DIR={}", out_dir);
    let work_dir = vec!["../../..", "../../../deps"];
    work_dir.iter().for_each(|dir_path| {
        let exe_dir = Path::new(&out_dir[..]).join(dir_path).canonicalize()
            .expect(&format!("失败：不能从 {} 推断出 {} 目录", out_dir, dir_path)[..]);
        symbolic_link_zlib1(&exe_dir);
    });
    let work_dir = vec!["../../../..", "../../.."];
    work_dir.iter().for_each(|dir_path| {
        let exe_dir = Path::new(&out_dir[..]).join(dir_path).canonicalize()
            .expect(&format!("失败：不能从 {} 推断出 {} 目录", out_dir, dir_path)[..]);
        symbolic_link_assets(&exe_dir);
    });
    generate_cpp_header_file(&out_dir[..]);
}
fn generate_cpp_header_file<'a>(out_dir: &'a str) {
    let cargo_name = env::var("CARGO_PKG_NAME")
        .expect("失败：环境变量`CARGO_PKG_NAME`未提供");
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("失败：环境变量`CARGO_MANIFEST_DIR`未提供");
    let cbindgen_toml = {
        let mut p = PathBuf::new();
        p.push(&cargo_manifest_dir[..]);
        p.push("cbindgen.toml");
        p
    };
    let c_header = {
        let mut p = PathBuf::new();
        p.push(out_dir);
        p.push(format!("../../../{}.h", cargo_name));
        p
    };
    let config = Config::from_file(cbindgen_toml)
        .expect("失败：解析`cbindgen.toml`配置文件");
    Builder::new().with_config(config)
        .with_crate(&cargo_manifest_dir[..])
        .generate().expect("失败：生成`Cpp`头文件")
        .write_to_file(c_header);
}
#[cfg(windows)]
fn symbolic_link_zlib1(exe_dir: &PathBuf) {
    let msys2_home = match env::var("MSYS2_HOME") {
        Ok(value) => value,
        Err(_) => {
            println!("cargo:warning=环境变量`MSYS2_HOME`没有提供，没有链接操作会被执行");
            return;
        }
    };
    println!("调试：MSYS2_HOME={}", msys2_home);
    println!("调试：EXE_DIR={}", exe_dir.display());
    if !exe_dir.is_dir() {
        println!("cargo:warning={} 不是一个目录", exe_dir.display());
        process::exit(1);
    }
    let zlib1_symbol = exe_dir.join("zlib1.dll");
    println!("调试：ZLIB1_EXE={}", zlib1_symbol.display());
    if zlib1_symbol.exists() {
        fs::remove_file(zlib1_symbol.clone())
            .expect(&format!("失败：不能删除原来的 {} 符号链接文件", zlib1_symbol.display())[..]);
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
    let zlib1_origin = bin_dir.join("zlib1.dll");
    println!("调试：ZLIB1_FILE={}", zlib1_origin.display());
    if !zlib1_origin.is_file() {
        println!("cargo:warning={} 不是一个文件", zlib1_origin.display());
        process::exit(1);
    }
    os::windows::fs::symlink_file(zlib1_origin.clone(), zlib1_symbol.clone())
        .expect(&format!("失败：不能创建文件链接 {} 指向 {}", zlib1_symbol.display(), zlib1_origin.display())[..]);
    println!("成功：能创建文件链接 {} 指向 {}", zlib1_symbol.display(), zlib1_origin.display());
}
#[cfg(not(windows))]
fn symbolic_link_zlib1(_: &PathBuf) {}
fn symbolic_link_assets(exe_dir: &PathBuf) {
    let mut assets_origin = env::current_dir().expect("工程里没有 assets 目录");
    assets_origin.push(ASSETS_DIR_NAME);
    let assets_symbol = exe_dir.join(ASSETS_DIR_NAME);
    if assets_symbol.exists() {
        remove_link(assets_symbol.clone())
            .expect(&format!("失败：不能删除原来的 {} 符号链接文件", assets_symbol.display())[..]);
    }
    make_link(&assets_origin, &assets_symbol)
        .expect(&format!("失败：不能创建文件链接 {} 指向 {}", assets_symbol.display(), assets_origin.display())[..]);
    println!("成功：能创建目录链接 {} 指向 {}", assets_symbol.display(), assets_origin.display());
    fn remove_link(assets_symbol: PathBuf) -> IoResult<()> {
        #[cfg(windows)]
        let result = fs::remove_dir(assets_symbol);
        #[cfg(not(windows))]
        let result = fs::remove_file(assets_symbol);
        result
    }
    fn make_link(assets_origin: &PathBuf, assets_symbol: &PathBuf) -> IoResult<()> {
        #[cfg(windows)]
        let result = os::windows::fs::symlink_dir(assets_origin.clone(), assets_symbol.clone());
        #[cfg(unix)]
        let result = os::unix::fs::symlink(assets_origin.clone(), assets_symbol.clone());
        #[cfg(linux)]
        let result = os::linux::fs::symlink(assets_origin.clone(), assets_symbol.clone());
        result
    }
}
