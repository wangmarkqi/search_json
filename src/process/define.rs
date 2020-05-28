use serde_json::Value;
use std::collections::HashMap;

pub const SEP_SPLIT: &'static str = "--->";
pub const SEP_ROOT: &'static str = "ROOT";
pub const SEP_ARR: &'static str = "[]";
pub const SEP_OBJ: &'static str = "{}";
pub const SEP_INDEX: &'static str = ">>>";
pub const SEP_KEY: &'static str = ":::";

#[derive(Debug)]
pub struct Tracker {
    pub route: String,
    pub content: Value,
}

impl Tracker {
    pub fn new(r: &str, c: &Value) -> Tracker {
        Tracker {
            route: r.to_string(),
            content: c.to_owned(),
        }
    }
}

pub const JS_OBJ: &'static str = "obj";
pub const JS_ARR: &'static str = "arr";
pub const JS_NUM: &'static str = "num";
pub const JS_STR: &'static str = "str";
pub const JS_ROOT: &'static str = "root";



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsNode {
    pub level: usize,
    pub value: String,
    pub id: i32,
}

impl JsNode {
    pub fn empty() -> JsNode {
        JsNode {
            level: 0,
            value: "".to_string(),
            id: -1,
        }
    }
    pub fn new(v: &str, lev: usize, _id: i32) -> JsNode {
        JsNode {
            level: lev,
            value: v.to_string(),
            id: _id,
        }
    }
    pub fn get_by_id(id: i32, dic: &HashMap<String, JsNode>) -> JsNode {
        for (_k, v) in dic.iter() {
            if id == v.id {
                return v.to_owned();
            }
        }
        JsNode::empty()
    }
    pub fn node2str(&self) -> (String, String) {
        let brace_r = r#"},"#.to_string();
        let brace_l = r#"{"#.to_string();
        let bracket_r = r#"],"#.to_string();
        let bracket_l = r#"["#.to_string();
        let valueclone=self.value.clone();
        if self.value == "ROOT{}" {
            return (brace_l, r#"}"#.to_string());
        }
        if self.value == "ROOT[]" {
            return (bracket_l, r#"]"#.to_string());
        }
        if self.value.ends_with(":::[]") {
            let k = valueclone.replace(":::[]","");
            let r = format!(r#""{}":["#,k);
            return (r, bracket_r.clone());
        }
        if self.value.ends_with(":::{}") {
            let k = valueclone.replace(":::{}","");
            let r = format!(r#""{}":{{"#,k);
            return (r, brace_r.clone());
        }
        if self.value.ends_with(">>>{}") {
            return (brace_l.clone(), brace_r.clone());
        }
        if self.value.ends_with(">>>[]") {
            return (bracket_l, bracket_r.clone());
        }
        if self.value.contains(SEP_KEY) {
            let kv :Vec<&str>= valueclone.split(SEP_KEY).collect();
            let r = format!(r#""{}":"{}","#,kv[0],kv[1]);
            return (r, "".to_string());
        }
        if self.value.contains(SEP_INDEX) {
            let iv :Vec<&str>= valueclone.split(SEP_INDEX).collect();
            let r = format!(r#""{}","#,iv[1]);
            return (r, "".to_string());
        }
        ("".to_string(),"".to_string())
    }
}
pub fn test(){
    let vs=[
        "ROOT{}",
        "members:::[]",
        "2>>>{}",
        "secretIdentity:::wangqi",
        "active:::true",
    ];

    for v in vs.iter(){
        let value=v.to_string();

        let node=JsNode::new(&value,1,4);
        dbg!(node.node2str());
    }

}

