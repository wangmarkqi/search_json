use crate::common::common::*;
use serde_json::Value;
use crate::common::strerr::StrError;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use futures::executor::block_on;
use crate::process::define::*;

pub fn get_js_value(js: &str) -> Value {
    let v = serde_json::from_str(&js);
    let  res: Value;
    match v {
        Ok(vv) => res = vv,
        Err(e) => {
            res = Value::Null;
        }
    }
    res
}

pub fn iter_all(t: Tracker,sender:Sender<Tracker>) {
    let v = t.content;
    match v {
        Value::Array(x) => {
            for (ii,vv) in x.iter().enumerate() {
                let r=format!("{}{}{}{}{}",&t.route,SEP_ARR,SEP_SPLIT,ii,SEP_INDEX);
                let tt = Tracker::new(&r,vv) ;

                iter_all(tt,sender.clone());
            }
        }
        Value::Object(dic) => {
            for (kk, vv) in dic.iter() {
                let r=format!("{}{}{}{}{}",&t.route,SEP_OBJ,SEP_SPLIT,kk,SEP_KEY);
                let tt = Tracker::new(&r,vv) ;
                iter_all(tt,sender.clone());
            }
        }
        Value::Null => {
            return;
        }
        Value::Number(n) => {
            let r=format!("{}{}",&t.route,n.to_string());
            let tt = Tracker::new(&r,&Value::Number(n)) ;
            sender.send(tt).unwrap();
            return;
        }
        Value::String(s) => {
            let r=format!("{}{}",&t.route,&s);
            let tt = Tracker::new(&r,&Value::String(s)) ;
            sender.send(tt).unwrap();
            return;
        }
        Value::Bool(b) => {
            let r=format!("{}{}",&t.route,b);
            let tt = Tracker::new(&r,&Value::Bool(b)) ;
            sender.send(tt).unwrap();
            return;
        }
        _ => print!("nomatch{}",v),
    }
}
pub fn flat_json(js:&str)->Vec<String>{
    let (sender, receiver):(Sender<Tracker>, Receiver<Tracker>) = channel();
    let v=get_js_value(js);

    let t=Tracker::new(SEP_ROOT,&v);

    thread::spawn(move||{
        iter_all(t,sender.clone());
    });

    let mut res=vec![];
     for received in receiver {
         res.push(received.route);
    }
    res
}
pub fn test() {
    // let path= "D://data/txt/tianyancha/baseinfoV3/北京百度网讯科技有限公司.txt";
    let path= "./test.json";
    let js = read_file_as_txt(&path);
    let res=flat_json(&js);


}
