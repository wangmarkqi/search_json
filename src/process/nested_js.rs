use crate::common::common::*;
use crate::process::define::*;
use crate::process::flat_js::flat_json;
use std::collections::HashMap;
// people.sort_by(|a, b| b.age.cmp(&a.age));
use petgraph::visit::Dfs;
use petgraph::graphmap::UnGraphMap;
use std::collections::VecDeque;

fn routes2vec(ts: &Vec<String>) -> Vec<Vec<String>> {
    let mut res = vec![];
    for s in ts.iter() {
        let rs: Vec<String> = s.split(SEP_SPLIT).map(|e| e.to_owned()).collect();
        res.push(rs.to_vec());
    }
    res
}

fn node_dic(rs: &Vec<Vec<String>>) -> HashMap<String, JsNode> {
    let mut id = 0;
    let mut node_dic = HashMap::new();
    for route in rs.iter() {
        let n = route.len();
        for i in 0..n {
            let k = &route[i];
            if !node_dic.contains_key(&k.to_string()) {
                let node = JsNode::new(k, i, id);
                node_dic.insert(k.to_string(), node);
                id = id + 1;
            }
        }
    }
    node_dic
}

fn edges_vec(rs: &Vec<Vec<String>>, dic: &HashMap<String, JsNode>) -> Vec<(i32, i32)> {
    let mut edges = vec![];
    for route in rs.iter() {
        let n = route.len();
        for i in 0..n - 1 {
            let k1 = &route[i];
            let id1 = dic[k1].id;
            let k2 = &route[i + 1];
            let id2 = dic[k2].id;
            edges.push((id1, id2));
        }
    }
    edges
}

fn _nest_json(edges: &Vec<(i32, i32)>, dic: &HashMap<String, JsNode>) -> String {
    let mut stack: VecDeque<String> = VecDeque::new();
    let mut js_str = r#""#.to_owned();
    let graph = UnGraphMap::<_, ()>::from_edges(edges);

    let mut dfs = Dfs::new(&graph, 0);
    let mut pre_level = 0 as usize;
    while let Some(nx) = dfs.next(&graph) {
        let node = JsNode::get_by_id(nx, dic);
        let (add, instack) = node.node2str();

        let cur_level = node.level;
        // 优先stack弹出来
        if pre_level > cur_level {
            let mut up = pre_level - cur_level;
            while up > 0 {
                let pop = stack.pop_front();
                if let Some(outstack) = pop {
                    // 把最后一个元素逗号去掉
                    js_str.pop();

                    js_str = js_str + &outstack;
                };
                up = up - 1;
            }
        }

        if instack != "" {
            stack.push_front(instack);
        }
        js_str = js_str + &add;
        pre_level = cur_level;
    }
    // 最后要把根目录的stack弹出来
    if let Some(outstack) = stack.pop_front() {
        // 把最后一个元素逗号去掉
        js_str.pop();
        js_str = js_str + &outstack;
    };
    js_str.to_string()
}

pub fn nest_json(data: &Vec<String>) -> String {
    let res = routes2vec(&data);
    let dic = node_dic(&res);
    let edges = edges_vec(&res, &dic);
    let res1 = _nest_json(&edges, &dic);
    res1
}

pub fn test() {
    let path = "./data/test.json";
    let js = read_file_as_txt(&path);
    let data = flat_json(&js);
    dbg!(data.clone());
    let res=nest_json(&data);
    dbg!(res);


}