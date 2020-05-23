
use std::env;
use std::path::PathBuf;
//加载libload so不需要buildrs，直接使用cc编译c源码是下面的方式build_c，这个是使用bindgen根据wrapp h生成rust接口文件bindings rs，编译需要设定库路径，运行如果是动态链接
// export LD_LIBRARY_PATH=你的库的路径:$LD_LIBRARY_PATH
// echo $LD_LIBRARY_PATH

fn main() {
    setEnv();
}

#[allow(dead_code)]
fn setEnv() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib = format!("{}/lib", root);
    env::set_var("LIBZMQ_PREFIX",lib.clone());
    println!("lib=={}",&lib);
    println!("cargo:warning=MESSAGE");
    print!("cargo:rustc-env=LIBZMQ_PREFIX={}",&lib);
}
