use petgraph::Graph;
use petgraph::visit::Dfs;

pub fn test() {
    let mut graph = Graph::new();
    let origin = graph.add_node("origin");
    let destination_1 = graph.add_node("1");
    let destination_2 = graph.add_node("2");
    let destination_3 = graph.add_node("3");
    let cost_1 = graph.add_edge(origin, destination_1, 250);
    let cost_2 = graph.add_edge(origin, destination_2, 1099);
    let cost_3 = graph.add_edge(destination_1, destination_3, 1099);


}
