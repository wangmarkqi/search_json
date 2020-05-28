pub mod common;
pub mod process;


/// get all path
pub fn get_all_path(root: String) -> Vec<String> {
    common::common::get_all_local_path(root)
}
/// flat deep nested json
/// # Examples
/// ```
///    let path= "./test.json";
///    let js = read_file_as_txt(&path);
///    let res=flat_json(&js);
///     dbg!(res);
/// ```
pub fn read_file_as_txt(file: &str)->String {
    common::common::read_file_as_txt(file)
}
/// flat deep nested json
/// # Examples
/// ```
///    let path= "./test.json";
///    let js = read_file_as_txt(&path);
///    let res=flat_json(&js);
///     dbg!(res);
/// ```
pub fn flat_json(js:&str)->Vec<String>{
    process::flat_js::flat_json(js)
}

/// nest vec string back to origin
/// # Examples
///
/// ```
///   let path = "./data/test.json";
///   let js = read_file_as_txt(&path);
///   let data = flat_json(&js);
///   dbg!(data.clone());
///   let res=nest_json(&data);
///   dbg!(res);
/// ```
pub fn nest_json(data: &Vec<String>) -> String {
    process::nested_js::nest_json(data)
}


