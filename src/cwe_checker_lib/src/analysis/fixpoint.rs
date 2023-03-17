//! Creating and computing generic fixpoint computations.
//!
//! For general information on dataflow analysis using fixpoint algorithms see [Wikipedia](https://en.wikipedia.org/wiki/Data-flow_analysis).
//!
//! # General implementation notes
//!
// 不动点问题定义为，一个图其中，每个结点n有一个值val(n)，作为偏序集表示为其中所有值的集合；每条边e规定了一个规则“e:value -> value”
// 如果找到了所有结点都有赋值了的位置（？）就找到了fixpoint，即所有边的e(val(start_node)) <= val(end_node)
// 通常要找的是最小不动点，即所有结点n的val(n)尽可能小，但不能小于起始值
// 在graph模块中描述的，边表示的是状态的转移或者（人为添加的）信息
//!
//! In the current implementation edge transition functions are also allowed to return `None`
//! to indicate that no information flows through the edge.
//! In such a case the value at the target node of the edge will not get updated.
//! For example, an analysis can use this to indicate edges that are never taken
//! and thus prevent dead code to affect the analysis.
//!
//! # How to compute the solution to a fixpoint problem
//!
// 需要一个Context对象，包含了计算不动点所需要的信息，例如图以及如何计算转换函数（transition functions），并不是实际的初始值；
// 利用它创建一个Computation对象，通过这个对象修改结点的值，来达到不动点计算的初始状态。
// Computation对象也包含在确定起始值后进行不动点计算以及接受结果的方法

use fnv::FnvHashMap;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{BTreeMap, BTreeSet};

/// The context of a fixpoint computation.
///
/// All trait methods have access to the FixpointProblem structure, so that context informations are accessible through it.
pub trait Context {
    /// the type of edge labels of the underlying graph
    type EdgeLabel: Clone;
    /// the type of node labels of the underlying graph
    type NodeLabel;
    /// The type of the value that gets assigned to each node.
    /// The values should form a partially ordered set.
    type NodeValue: PartialEq + Eq + Clone;

    /// Get the graph on which the fixpoint computation operates.
    fn get_graph(&self) -> &DiGraph<Self::NodeLabel, Self::EdgeLabel>;

    /// This function describes how to merge two values
    fn merge(&self, val1: &Self::NodeValue, val2: &Self::NodeValue) -> Self::NodeValue;

    /// This function describes how the value at the end node of an edge is computed from the value at the start node of the edge.
    /// The function can return None to indicate that no end value gets generated through this edge.
    /// E.g. In a control flow graph, if the edge cannot be taken for the given start value, this function should return None.
    fn update_edge(&self, value: &Self::NodeValue, edge: EdgeIndex) -> Option<Self::NodeValue>;
}

/// The computation struct contains an intermediate result of a fixpoint computation
/// and provides methods for continuing the fixpoint computation
/// or extracting the (intermediate or final) results.
///
/// # Usage
///
/// ```ignore
/// let mut computation = Computation::new(context, optional_default_node_value);
///
/// // set starting node values with computation.set_node_value(..)
/// // ...
///
/// computation.compute();
///
/// // get the resulting node values
/// if let Some(node_value) = computation.get_node_value(node_index) {
///     // ...
/// };
/// ```
pub struct Computation<T: Context> {
    /// The context object needed for the fixpoint computation
    fp_context: T,
    /// maps a node index to its priority (higher priority nodes get stabilized first)
    node_priority_list: Vec<usize>,
    /// maps a priority to the corresponding node index
    priority_to_node_list: Vec<NodeIndex>,
    /// The worklist contains the priority numbers (not the node indices!) of nodes marked as not yet stabilized.
    worklist: BTreeSet<usize>,
    /// The internal map containing all known node values.
    node_values: FnvHashMap<NodeIndex, T::NodeValue>,
}

impl<T: Context> Computation<T> {
    /// Create a new fixpoint computation from a fixpoint problem, the corresponding graph
    /// and a default value for all nodes if one should exists.
    pub fn new(fp_context: T, default_value: Option<T::NodeValue>) -> Self {
        let graph = fp_context.get_graph();
        // order the nodes in weak topological order
        let priority_sorted_nodes: Vec<NodeIndex> = petgraph::algo::kosaraju_scc(&graph)
            .into_iter()
            .flatten()
            .collect();
        //Return a vector where each element is a strongly connected component (scc).
        //The order of node ids within each scc is arbitrary, but the order of the sccs is their postorder (reverse topological sort).

        Self::from_node_priority_list(fp_context, default_value, priority_sorted_nodes)
    }

    /// Create a new fixpoint computation from a fixpoint problem, an optional default value
    /// and the list of nodes of the graph ordered by the priority for the worklist algorithm.
    /// The worklist algorithm will try to stabilize the nodes with a higher index
    /// in the `priority_sorted_nodes` array before those with a lower index.
    pub fn from_node_priority_list(
        fp_context: T,
        default_value: Option<T::NodeValue>,
        priority_sorted_nodes: Vec<NodeIndex>,
    ) -> Self {
        //重新排序向量priority_sorted_nodes，下标大（值小）的放在后面
        let mut node_to_index = BTreeMap::new();
        for (i, node_index) in priority_sorted_nodes.iter().enumerate() {   //enumerate遍历元素和下标
            node_to_index.insert(node_index, i);
        }
        let node_priority_list: Vec<usize> = node_to_index.values().copied().collect();
        let mut worklist = BTreeSet::new();
        // If a default value exists, all nodes are added to the worklist. If not, the worklist is empty
        let mut node_values: FnvHashMap<NodeIndex, T::NodeValue> = FnvHashMap::default();
        if let Some(default) = default_value {//？
            for i in 0..priority_sorted_nodes.len() {
                worklist.insert(i);
                node_values.insert(NodeIndex::new(i), default.clone());
            }
        }
        Computation {
            fp_context,
            node_priority_list,
            priority_to_node_list: priority_sorted_nodes,
            worklist,
            node_values,
        }
    }

    /// Get the value of a node.
    pub fn get_node_value(&self, node: NodeIndex) -> Option<&T::NodeValue> {
        self.node_values.get(&node)
    }

    /// Set the value of a node and mark the node as not yet stabilized.
    pub fn set_node_value(&mut self, node: NodeIndex, value: T::NodeValue) {
        self.node_values.insert(node, value);
        self.worklist.insert(self.node_priority_list[node.index()]);
    }

    /// Merge the value at a node with some new value.
    fn merge_node_value(&mut self, node: NodeIndex, value: T::NodeValue) {
        if let Some(old_value) = self.node_values.get(&node) {  //?
            let merged_value = self.fp_context.merge(&value, old_value);
            if merged_value != *old_value {
                self.set_node_value(node, merged_value);
            }
        } else {//找不到node对应的值
            self.set_node_value(node, value);
        }
    }

    /// Compute and update the value at the end node of an edge.
    fn update_edge(&mut self, edge: EdgeIndex) {
        let (start_node, end_node) = self
            .fp_context
            .get_graph()
            .edge_endpoints(edge)//找到边的起始和终点
            .expect("Edge not found");
        if let Some(start_val) = self.node_values.get(&start_node) {
            if let Some(new_end_val) = self.fp_context.update_edge(start_val, edge) {
                self.merge_node_value(end_node, new_end_val);
            }
        }
    }

    /// Update all outgoing edges of a node.//出边
    fn update_node(&mut self, node: NodeIndex) {
        let edges: Vec<EdgeIndex> = self//遍历这个节点的所有边
            .fp_context
            .get_graph()
            .edges(node)
            .map(|edge_ref| edge_ref.id())
            .collect();
        for edge in edges {
            self.update_edge(edge);
        }
    }

    /// Remove the highest priority node from the internal worklist and return it.
    fn take_next_node_from_worklist(&mut self) -> Option<NodeIndex> {
        if let Some(priority) = self.worklist.iter().next_back().cloned() {
            let priority = self.worklist.take(&priority).unwrap();
            Some(self.priority_to_node_list[priority])
        } else {
            None
        }
    }

    /// Compute the fixpoint of the fixpoint problem.
    /// Each node will be visited at most max_steps times.
    /// If a node does not stabilize after max_steps visits, the end result will not be a fixpoint but only an intermediate result of a fixpoint computation.
    pub fn compute_with_max_steps(&mut self, max_steps: u64) {
        let mut steps = vec![0; self.fp_context.get_graph().node_count()]; //nodecount：全图结点数量
        let mut non_stabilized_nodes = BTreeSet::new();
        while let Some(priority) = self.worklist.iter().next_back().cloned() {
            //.next_back，从后迭代，碰到.next的指针停止
            let priority = self.worklist.take(&priority).unwrap(); //.take：移除并赋值。
            let node = self.priority_to_node_list[priority];    //priority_to_node_list
            if steps[node.index()] < max_steps {    //每个节点给100次
                steps[node.index()] += 1;
                self.update_node(node); //更新node，过程间分析
                
            } else {
                non_stabilized_nodes.insert(priority);  //worklist算法
            }
        }
        // After the algorithm finished, the new worklist is the list of non-stabilized nodes
        self.worklist = non_stabilized_nodes;
    }

    /// Compute the fixpoint of the fixpoint problem.
    /// If the fixpoint algorithm does not converge to a fixpoint, this function will not terminate.
    pub fn compute(&mut self) {
        while let Some(node) = self.take_next_node_from_worklist() {
            self.update_node(node);
        }
    }

    /// Get a reference to the internal map where one can look up the current values of all nodes
    pub fn node_values(&self) -> &FnvHashMap<NodeIndex, T::NodeValue> {
        &self.node_values
    }

    /// Get a reference to the underlying graph
    pub fn get_graph(&self) -> &DiGraph<T::NodeLabel, T::EdgeLabel> {
        self.fp_context.get_graph()
    }

    /// Get a reference to the underlying context object
    pub fn get_context(&self) -> &T {
        &self.fp_context
    }

    /// Returns `True` if the computation has stabilized, i.e. the internal worklist is empty.
    pub fn has_stabilized(&self) -> bool {
        self.worklist.is_empty()
    }

    /// Return a list of all nodes which are marked as not-stabilized
    pub fn get_worklist(&self) -> Vec<NodeIndex> {
        self.worklist
            .iter()
            .map(|priority| self.priority_to_node_list[*priority])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FPContext {
        graph: DiGraph<(), u64>,
    }

    impl Context for FPContext {
        type EdgeLabel = u64;
        type NodeLabel = ();
        type NodeValue = u64;

        fn get_graph(&self) -> &DiGraph<(), u64> {
            &self.graph
        }

        fn merge(&self, val1: &Self::NodeValue, val2: &Self::NodeValue) -> Self::NodeValue {
            std::cmp::min(*val1, *val2)
        }

        fn update_edge(&self, value: &Self::NodeValue, edge: EdgeIndex) -> Option<Self::NodeValue> {
            Some(value + self.graph.edge_weight(edge).unwrap())
        }
    }

    #[test]
    fn fixpoint() {
        let mut graph: DiGraph<(), u64> = DiGraph::new();
        for _i in 0..101 {
            graph.add_node(());
        }
        for i in 0..100 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), i as u64 % 10 + 1);
        }
        for i in 0..10 {
            graph.add_edge(NodeIndex::new(i * 10), NodeIndex::new(i * 10 + 5), 0);
        }
        graph.add_edge(NodeIndex::new(100), NodeIndex::new(0), 0);

        let mut solution = Computation::new(FPContext { graph }, None);
        solution.set_node_value(NodeIndex::new(0), 0);
        solution.compute_with_max_steps(20);

        assert_eq!(30, *solution.get_node_value(NodeIndex::new(9)).unwrap());
        assert_eq!(0, *solution.get_node_value(NodeIndex::new(5)).unwrap());
    }

    #[test]
    fn fixpoint_with_default_value() {
        let mut graph: DiGraph<(), u64> = DiGraph::new();
        for _i in 0..101 {
            graph.add_node(());
        }
        for i in 0..100 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), i as u64 % 10 + 1);
        }
        for i in 0..10 {
            graph.add_edge(NodeIndex::new(i * 10), NodeIndex::new(i * 10 + 5), 0);
        }

        let mut solution = Computation::new(FPContext { graph }, Some(100));
        solution.set_node_value(NodeIndex::new(10), 0);
        solution.compute_with_max_steps(20);

        assert_eq!(100, *solution.get_node_value(NodeIndex::new(0)).unwrap());
        assert_eq!(3, *solution.get_node_value(NodeIndex::new(12)).unwrap());
    }

    #[test]
    fn worklist_node_order() {
        let mut graph: DiGraph<(), u64> = DiGraph::new();
        for _i in 0..21 {
            graph.add_node(());
        }
        for i in 1..19 {
            graph.add_edge(NodeIndex::new(0), NodeIndex::new(i), 1);
        }
        for i in 1..19 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(19), 1);
        }
        graph.add_edge(NodeIndex::new(19), NodeIndex::new(20), 1);
        let mut computation = Computation::new(
            FPContext {
                graph: graph.clone(),
            },
            Some(1),
        );
        assert!(computation.node_priority_list[0] > computation.node_priority_list[1]);
        assert!(computation.node_priority_list[1] > computation.node_priority_list[19]);
        assert!(computation.node_priority_list[19] > computation.node_priority_list[20]);
        // assert that the nodes have the correct priority ordering
        assert_eq!(
            computation.take_next_node_from_worklist(),
            Some(NodeIndex::new(0))
        );
        for _i in 1..19 {
            assert!(computation.take_next_node_from_worklist().unwrap().index() < 19);
        }
        assert_eq!(
            computation.take_next_node_from_worklist(),
            Some(NodeIndex::new(19))
        );
        assert_eq!(
            computation.take_next_node_from_worklist(),
            Some(NodeIndex::new(20))
        );
    }
}
