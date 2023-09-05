use crate::dataclass::graphs::TGraph;
use crate::rules::predicates::graph_predicate::*;

#[allow(dead_code)]
pub fn line_length<P, T: TGraph<P> + std::fmt::Debug>(
    graphs: &Vec<T>,
    constraint_length: usize,
) -> bool {
    #[allow(unused_assignments)]
    let mut is_satisfy = true;

    is_satisfy = constraint_to_graph_length(graphs, constraint_length);
    return is_satisfy;
}
