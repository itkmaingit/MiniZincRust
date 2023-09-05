use crate::dataclass::graphs::TGraph;

#[allow(dead_code)]
pub fn constraint_to_graph_length<P, T: TGraph<P>>(
    graphs: &Vec<T>,
    constraint_length: usize,
) -> bool {
    let mut is_satisfy = true;
    for graph in graphs.iter() {
        if graph.length() != constraint_length {
            is_satisfy = false;
        }
    }
    return is_satisfy;
}
