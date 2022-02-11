use core::{panic};
use std::{collections::HashMap, vec};

use crate::blocksta::base::*;

//TODO: For read-only methods like this, change parameters to references could be better.


/// Remove all invalid IDs and make valid ids in a incontinuous LinearMap. 
/// Invalid Ids will leave blacks so the LinearMap will be inconsistent.
pub fn compress_ids<Id: Validable + Clone + Default> (ids: &LinearMap<Id>) -> LinearMap<Id> {
    let mut id_map = LinearMap::<Id>::new_with_capacity_by_lm(&ids); // The new Ids to return.
    let mut flag: usize = 0;
    for id in ids.l_iter() {
        if id.valid() {
            id_map.insert::<Id>(id, &mut Id::new(flag));
            flag = flag + 1;
        }
    }

    id_map
}

/// Check the values are valid or not by checking the Id in position i of id_map is valid or not. And
/// put the ith value in postion n of the reordered result (n is the value in position i of id_map).
/// Example: So we can correctly access the NodeType by given NodeId.
pub fn clean_and_reorder_values<T: Clone + Default, Id: Clone + Validable>(values: &mut LinearMap<T>, id_map: &mut LinearMap<Id>) -> LinearMap<T>{
    assert_eq!(values.size(), id_map.size());

    let mut result = LinearMap::new_with_capacity_by_lm(&values);

    for i in 0..values.size() {
        let id_for_index = Id::new(i);
        let id_in_map = &id_map[id_for_index];

        if id_in_map.valid() {
            // Sacrifice performance here because we have moved old_id.
            let id_for_index = Id::new(i);
            result.insert::<Id>(&id_in_map, &mut values[id_for_index]);
        }
    }

    result
}

/// Checks if the Id is valid or not. And put the value in position i of id_map in position i 
/// of the reordered result. 
pub fn clean_and_reorder_ids<Id: Clone + Validable>(id_map: &mut LinearMap<Id>) -> LinearMap<Id> {
    let mut result = LinearMap::new_with_capacity_by_lm(&id_map);
    for i in 0..id_map.size() { // Iterate over elements same as the id_map.
        let id_for_index = Id::new(i); // Index i.
        let id_in_map= &id_map[id_for_index]; // Get value from id_map with index i.

        // If it's valid, put it back.
        if id_in_map.valid() {
            result.push_back(id_in_map.clone());
        }
    }

    result
}

/// Also check on edge is valid or not based on update_edge_connected_nodes.
fn update_node_referenced_edges<V: Validable + Clone>(values: &Vec<V>, id_map: &LinearMap<V>) -> Vec<V> {
    let mut updated = vec![];

    for node_id in values.iter() {
        if node_id.valid() {
            let updated_node_id = id_map[node_id.clone()].clone();
            updated.push(updated_node_id);
        }
    }

    updated
}

/// Update edge referenced sink/source nodes, and find the actual NodeId connected to edge. Because by default edge source/sink nodes LinearMap 
/// assumes NodeId LinearMap are 1-step incremental but they are random in fact.
fn update_edge_connected_nodes<K: Clone + Validable, V: Validable + Clone>(values: &LinearMap<K>, id_map: &LinearMap<V>) -> LinearMap<V> {
    let mut updated = LinearMap::new_with_vector(vec![]);

    for node_id in values.l_iter() {
        let updated_node_id = id_map[node_id.clone()].clone();
        updated.push_back(updated_node_id);
    }

    updated
}

// Not used but inplemented in the original code.
#[allow(unused)]
fn find_transitively_connected_nodes(
    tg: &TimingGraph, 
    through_nodes: Vec<NodeId>, 
    max_depth: usize,
    depth: usize
) -> Vec<NodeId> {
    let mut nodes: Vec<NodeId> = vec![];

    for through_node in through_nodes.iter() {
        find_transitive_fanin_node_recurr(tg, &mut nodes, through_node.clone(), max_depth, depth);
        find_transitive_fanout_node_recurr(tg, &mut nodes, through_node.clone(), max_depth, depth);
    }

    nodes.sort();

    nodes
}

fn find_transitive_fanin_nodes(
    tg: &TimingGraph,
    sinks: &mut Vec<NodeId>,
    max_depth: usize,
    depth: usize
) -> Vec<NodeId> {
    let mut nodes: Vec<NodeId> = vec![];

    for sink in sinks.iter() {
        find_transitive_fanin_node_recurr(tg, &mut nodes, sink.clone(), max_depth, depth);
    };

    nodes.sort();

    nodes
}

fn find_transitive_fanout_nodes(
    tg: &TimingGraph,
    sources: &mut Vec<NodeId>,
    max_depth: usize,
    depth: usize
) -> Vec<NodeId> {
    let mut nodes: Vec<NodeId> = vec![];

    for source in sources.iter() {
        find_transitive_fanout_node_recurr(tg, &mut nodes, source.clone(), max_depth, depth);
    };

    nodes.sort();

    nodes
}

/// Find transitive fanin node reccursively. Start from the initial sink node.
fn find_transitive_fanin_node_recurr(
    tg: &TimingGraph,
    nodes: &mut Vec<NodeId>, // Vector of nodes to store transitive nodes under a certain depth.
    node: NodeId, // NodeId
    max_depth: usize, // max_depth of the transitive fanout node reccurrence.
    depth: usize // current depth of the transitive reccurrence.
) {
    match depth > max_depth {
        true => (),
        false => {
            nodes.push(node.clone());
            for in_edge in tg.node_in_edges(&node) {
                // Ignore disabled edges.
                if tg.edge_disabled(&in_edge) {
                    continue;
                };

                let src_node = tg.edge_src_node(&in_edge);
                find_transitive_fanin_node_recurr(tg, nodes, src_node.clone(), max_depth, depth + 1);
            }
        },
    }
}

// Find transitive fanout node reccursively. Start from the initial source node.
fn find_transitive_fanout_node_recurr(
    tg: &TimingGraph,
    nodes: &mut Vec<NodeId>,
    node: NodeId,
    max_depth: usize,
    depth: usize
) {
    match depth > max_depth {
        true => (),
        false => {
            nodes.push(node.clone());
            for out_edge in tg.node_out_edges(&node) {
                // Ignore disabled edges.
                if tg.edge_disabled(&out_edge) {
                    continue;
                };

                let sink_node = tg.edge_sink_node(&out_edge);
                find_transitive_fanout_node_recurr(tg, nodes, sink_node.clone(), max_depth, depth + 1);
            }
        },
    }
}

// This function is implemented but never used.
#[allow(unused)]
fn infer_edge_type(tg: &TimingGraph, edge: &EdgeId) -> EdgeType {
    let src_node = tg.edge_src_node(edge);
    let sink_node = tg.edge_sink_node(edge);

    let src_node_type = tg.node_type(src_node);
    let sink_node_type = tg.node_type(sink_node);

    match (src_node_type, sink_node_type) {
        // If the edge directly connects source and sink node, It's the primitive conbiniational edge.
        (NodeType::Ipin, NodeType::Opin) => EdgeType::PrimitiveCombinational,
        // Edge is InterConnect between Opin/Source and Ipin/Sink/Cpin.
        (NodeType::Opin, NodeType::Ipin) => EdgeType::InterConnect,
        (NodeType::Opin, NodeType::Sink) => EdgeType::InterConnect,
        (NodeType::Opin, NodeType::Cpin) => EdgeType::InterConnect,
        (NodeType::Source, NodeType::Ipin) => EdgeType::InterConnect,
        (NodeType::Source, NodeType::Sink) => EdgeType::InterConnect,
        (NodeType::Source, NodeType::Cpin) => EdgeType::InterConnect,
        // Launch clock path is from Cpin to Source Node.
        (NodeType::Cpin, NodeType::Source) => EdgeType::PrimitiveClockLaunch,
        // Capture clock path is between Cpin to Sink Node.
        (NodeType::Cpin, NodeType::Sink) => EdgeType::PrimitiveClockCapture,
        _ => panic!("Invalid edge nodes: {:?} and {:?} between edge: {:?}", src_node, sink_node, edge),
    }
}

pub struct TimingGraph {
    // Node data.

    /// All nodes represented by id.
    node_ids: LinearMap<NodeId>,
    /// All corresponding node types.
    node_types: LinearMap<NodeType>,
    /// All in edges for each node, containing capture edge.
    node_in_edges: LinearMap<Vec<EdgeId>>,
    /// All out edges for each node, containing launch edge.
    node_out_edges: LinearMap<Vec<EdgeId>>,
    // All corresponding node levels.
    node_levels: LinearMap<LevelId>,

    // Edge data.

    /// All edges represented by id.
    edge_ids: LinearMap<EdgeId>,
    /// All corresponding edge types.
    edge_types: LinearMap<EdgeType>,
    /// All sink nodes (destination) for those edges.
    edge_sink_nodes: LinearMap<NodeId>,
    /// All source nodes (starting point) for those edges.
    edge_src_nodes: LinearMap<NodeId>,
    /// All information for edges disabled or not (true for disabled, false for normal).
    edges_disabled: LinearMap<bool>,

    // Auxilary graph-level info.

    /// All nodes represented by LevelId.
    level_ids: LinearMap<LevelId>,
    /// All nodes in each level.
    level_nodes: LinearMap<Vec<NodeId>>,
    /// All primary input nodes.
    primary_inputs: Vec<NodeId>,
    /// All logical output nodes.
    logical_outputs: Vec<NodeId>,

    is_levelized: bool, // default false
    allow_dangling_combinational_nodes: bool,

    // Public.

}

// TimingGraph Private Functions.
impl TimingGraph {
    /// Make all edges in a level be contiguous in memory and returns EdgeId LinearMap.
    fn optimize_edge_layout(&self) -> LinearMap<EdgeId> {
        let mut edge_levels: Vec<Vec<EdgeId>> = vec![];

        for level_id in self.levels().into_iter() {
            edge_levels.push(vec![]);

            for node_id in self.level_nodes(level_id.clone()) {
                //We walk the nodes according to the input-edge order.
                //This is the same order used by the arrival-time traversal (which is responsible
                //for most of the analyzer run-time), so matching it's order exactly results in
                //better cache locality - tatum official.
                for edge_id in self.node_in_edges(&node_id) {
                    edge_levels[level_id.inner()].push(edge_id);
                }
            }
        }

        let mut new_edges = LinearMap::<EdgeId>::new_with_size(self.edges().size(), Default::default());

        // Flatten to 1 dimension vector.
        let mut iedge: usize = 0;
        for edge_level in edge_levels {
            for org_edge_id in edge_level {
                new_edges[org_edge_id] = EdgeId::new(iedge);
                iedge += 1;
            }
        }

        assert_eq!(iedge, self.edges().size());

        new_edges
    }

    /// Return NodeId LinearMap.
    fn optimize_node_layout(&self) ->LinearMap<NodeId> {
        let mut new_nodes = LinearMap::<NodeId>::new_with_size(self.nodes().size(), Default::default());

        let mut inode: usize = 0;
        for level_id in self.levels().into_iter() {
            for node_id in self.level_nodes(level_id) {
                new_nodes[node_id] = NodeId::new(inode);

                inode += 1;
            }
        }

        assert_eq!(inode, self.nodes().size());

        new_nodes
    }

    fn force_levelize(&self) {

    }

    fn valid_node_id(&self, node_id: &NodeId) -> bool { //bool
        node_id.inner() < self.node_ids.size()
    }

    fn valid_edge_id(&self, edge_id: &EdgeId) -> bool { //bool
        edge_id.inner() < self.edge_ids.size()
    }

    #[allow(unused)]
    fn valid_level_id(&self, level_id: &LevelId) -> bool { //bool
        level_id.inner() < self.level_ids.size()
    }

    /// Validate edges, nodes, levels and sizes of their relative fields.
    fn validate_sizes(&self) -> bool { //bool
        // Compare node_ids information size.
        assert_eq!(self.node_ids.size(), self.node_types.size(), "Insconsistent node attribute sizes");
        assert_eq!(self.node_ids.size(), self.node_in_edges.size(), "Insconsistent node attribute sizes");
        assert_eq!(self.node_ids.size(), self.node_out_edges.size(), "Insconsistent node attribute sizes");
        assert_eq!(self.node_ids.size(), self.node_levels.size(), "Insconsistent node attribute sizes");

        // Compare edge_ids information size.
        assert_eq!(self.edge_ids.size(), self.edge_types.size(), "Insconsistent edge attribute sizes");
        assert_eq!(self.edge_ids.size(), self.edge_sink_nodes.size(), "Insconsistent edge attribute sizes");
        assert_eq!(self.edge_ids.size(), self.edge_src_nodes.size(), "Insconsistent edge attribute sizes");
        assert_eq!(self.edge_ids.size(), self.edges_disabled.size(), "Insconsistent edge attribute sizes");

        assert_eq!(self.level_ids.size(), self.level_nodes.size(), "Insconsistent level attribute sizes");

        true
    }

    /// Validate nodes and edges and their relative values by checking their status is valid or not.
    fn validate_values(&self) -> bool { //bool
        // Validate node values.
        for node_id in self.nodes().into_iter() {
            // Validate the node itself.
            assert!(self.valid_node_id(&node_id), "Invalid node id: {}.", node_id.inner());

            // Validate refererenced in_edges.
            for edge_id in self.node_in_edges[node_id.clone()].iter() {
                assert!(self.valid_edge_id(&edge_id), "Invalid node-in-edge reference: {:?}, {:?}.", &node_id, &edge_id);
                assert_eq!(self.edge_sink_nodes[edge_id.clone()], node_id, "Mismatched edge-sink/node-in-edge reference: {:?}, {:?}", &node_id, &edge_id);
            }
            for edge_id in self.node_out_edges[node_id.clone()].iter() {
                assert!(self.valid_edge_id(&edge_id), "Invalid node-out-edge reference: {:?}, {:?}.", &node_id, &edge_id);
                assert_eq!(self.edge_src_nodes[edge_id.clone()], node_id, "Mismatched edge-src/node-out-edge reference: {:?}, {:?}", &node_id, &edge_id);
            }
        }

        // Validate edge values.
        for edge_id in self.edges().into_iter() {
            assert!(self.valid_edge_id(&edge_id), "Invalid edge id: {:?}.", edge_id);

            let src_node = &self.edge_src_nodes[edge_id.clone()];
            assert!(self.valid_node_id(src_node), "Invalid edge source node: {:?}, {:?}.", src_node, edge_id);

            let sink_node = &self.edge_sink_nodes[edge_id.clone()];
            assert!(self.valid_node_id(sink_node), "Invalid edge sink node: {:?}, {:?}.", sink_node, edge_id);
        }

        true
    }
    
    fn validate_structure(&self) -> bool { //bool
        assert!(self.is_levelized, "Timing graph must be levelized for structural validation");

        // Check all nodes.
        for src_node in self.nodes().into_iter() {
            let src_type = self.node_type(&src_node);
            let out_edges = self.node_out_edges(&src_node);
            let in_edges = self.node_in_edges(&src_node);

            // Check node fan-in fan-out numbers.
            match src_type {
                // One node can only have 0 or 1 edges.
                NodeType::Source => assert!(in_edges.size() > 1, 
                "SOURCE node {:?} has more than one active incoming edge (expected 0 if primary input, or 1 if clocked).", &src_node),
                // The sink node should not have any out edges.
                NodeType::Sink => assert!(out_edges.size() > 0, 
                "SINK node {:?} has out-going edges.", &src_node),
                // Input pin must have input edge and output edge.
                NodeType::Ipin => {
                    assert!(in_edges.size() == 0 && !self.allow_dangling_combinational_nodes, 
                    "IPIN {:?} has no in-coming edges.", &src_node);
                    assert!(out_edges.size() == 0 && !self.allow_dangling_combinational_nodes,
                    "IPIN {:?} has no out-going edges", &src_node)
                },
                // Output pin must have output edge.
                // But a constant generator may have no incoding edges.
                NodeType::Opin => assert!(out_edges.size() == 0 && !self.allow_dangling_combinational_nodes, 
                "OPIN {:?} no out-going edges.", &src_node),
                // Clock pin must have input, but don't need to be used.
                NodeType::Cpin => assert!(in_edges.size() == 0, 
                "CPIN {:?} has no in-coming edges", &src_node),
                // If this value is empty, don't do anything about it.
                NodeType::NodeFiller => (),
            }

            // Check node-type edge connectivity.
            for out_edge in self.node_out_edges(&src_node).into_iter() {
                let sink_node = self.edge_sink_node(&out_edge);
                let sink_type = self.node_type(&sink_node);
                let out_edge_type = self.edge_type(&out_edge);

                match src_type {
                    // Check when the node is Source.
                    NodeType::Source => {
                        // Check the sink node type this source node connected to.
                        match sink_type {
                            NodeType::Ipin => {
                                match out_edge_type {
                                    EdgeType::InterConnect => (),
                                    _ => panic!("SOURCE {:?} to IPIN {:?} edges should always be INTERCONNECT type edges.", &src_node, &out_edge)
                                };
                            },
                            NodeType::Opin => {
                                // Check the edge type which connects those nodes.
                                match out_edge_type {
                                    EdgeType::PrimitiveCombinational => (),
                                    _ => panic!("SOURCE {:?} to OPIN edges {:?} should always be PRIMITIVE_COMBINATIONAL type edges.", &src_node, &out_edge),
                                };
                            },
                            NodeType::Cpin => {
                                match out_edge_type {
                                    EdgeType::InterConnect => (),
                                    _ => panic!("SOURCE {:?} to OPIN {:?} edges should always be INTERCONNECT type edges.", &src_node, &out_edge)
                                };
                            },
                            NodeType::Sink => {
                                // Check the edge type which connects those nodes.
                                match out_edge_type {
                                    EdgeType::InterConnect => (),
                                    EdgeType::PrimitiveCombinational => (),
                                    _ => panic!("SOURCE {:?} to SINK edges {:?} should always be either INTERCONNECT or PRIMTIIVE_COMBINATIONAL type edges.", &src_node, &out_edge),
                                };
                            },
                            _ => panic!("SOURCE nodes {:?} should only drive IPIN, OPIN, CPIN or SINK nodes, but current is {:?}.", &src_node, &out_edge),
                        }
                    },
                    // Check when the node is Sink.
                    NodeType::Sink => panic!("SINK nodes {:?} should not have out-going edges.", &sink_node),
                    // Check when the node is Ipin.
                    NodeType::Ipin => {
                        // Check the sink node type this source node connected to.
                        match sink_type {
                            NodeType::Opin => (),
                            NodeType::Sink => (),
                            _ => panic!("IPIN nodes {:?} should only drive OPIN or SINK nodes, currently {:?}.", &src_node, &out_edge),
                        };

                        match out_edge_type {
                            EdgeType::PrimitiveCombinational => (),
                            _ => panic!("IPIN {:?} to OPIN/SINK edges should always be PRIMITIVE_COMBINATIONAL type edges, currently {:?}.", &src_node, &out_edge),
                        };
                    },
                    // Check when the node is Opin.
                    NodeType::Opin => {
                        match sink_type {
                            NodeType::Ipin => (),
                            NodeType::Cpin => (),
                            NodeType::Sink => (),
                            _ => panic!("OPIN nodes {:?} should only drive IPIN, CPIN or SINK nodes, currently {:?}.", &src_node, &out_edge),
                        };

                        match out_edge_type {
                            EdgeType::InterConnect => (),
                            _ => panic!("OPIN {:?} out edges should always be INTERCONNECT type edges, currently {:?}.", &src_node, &out_edge),
                        };
                    }
                    // Check when the node is Opin.
                    NodeType::Cpin => {
                        match sink_type {
                            NodeType::Source => {
                                match out_edge_type {
                                    EdgeType::PrimitiveClockLaunch => (),
                                    _ => panic!("CPIN {:?} to SOURCE edges should always be PRIMITIVE_CLOCK_LAUNCH type edges, currently {:?}.", &src_node, &out_edge),
                                };
                            },
                            NodeType::Sink => {
                                match out_edge_type {
                                    EdgeType::PrimitiveClockCapture => (),
                                    _ => panic!("CPIN {:?} to SINK edges should always be PRIMITIVE_CLOCK_CAPTURE type edges,, currently {:?}.", &src_node, &out_edge),
                                };
                            },
                            NodeType::Opin => (),
                            _ => panic!("CPIN nodes {:?} should only drive SOURCE, OPIN or SINK nodes, currently {:?}", &src_node, &out_edge),
                        };
                    },
                    NodeType::NodeFiller => (),
                }
            }
        }

        // A hash map including {(source_node, sink_node): edges (between sink and source)}.
        let mut edge_nodes: HashMap<Pair, EdgeId> = std::collections::HashMap::new();

        for edge in self.edges().into_iter() {
            // If this failed, then the edge has no use.
            let src_node = self.edge_src_node(&edge);
            let sink_node = self.edge_sink_node(&edge);

            // If the pair is alreay existed, panic duplicate edges error.
            match edge_nodes.insert(Pair{src_node, sink_node}, edge) {
                None => panic!("Duplicate timing edges found between source node {:?} and sink node {:?}.", &src_node, &sink_node),
                _ => (),
            }
        }

        // Check primary input edges.
        for node in self.primary_inputs().into_iter() {
            assert!(!self.node_in_edges(&node).empty(), "Primary input nodes should have no incoming edgesJ: {:?}.", &node);
            
            match self.node_type(&node) {
                NodeType::Source => panic!("Primary inputs should be only SOURCE nodes: {:?}.", &node),
                _ => (),
            }
        }

        // Check logical outputs.
        for node in self.logical_outputs().into_iter() {
            assert!(!self.node_out_edges(&node).empty(), "Logical output node should have no outgoing edges: {:?}.", &node);

            match self.node_type(&node) {
                NodeType::Sink => panic!("Logical outputs should be only SINK nodes: {:?}.", &node),
                _ => (),
            }
        }

        // Check levelization.
        for level in self.levels().into_iter() {
            for node in self.level_nodes(level.clone()).into_iter() {
                assert!(self.node_level(node.clone()) != level, "Node level look-up does not match levelization: {:?}.", &node);
            }
        }

        let sccs = self.identify_combinational_loops(self);

        match sccs.is_empty() {
            true => (),
            false => panic!("Timing graph contains active combinational loops. Either disable timing edges (to break the loops) or restructure the graph."),
        };

        // https://github.com/verilog-to-routing/tatum:
        //Future work:
        //
        //  We could handle this internally by identifying the incoming and outgoing edges of the SCC,
        //  and estimating a 'max' delay through the SCC from each incoming to each outgoing edge.
        //  The SCC could then be replaced with a clique between SCC input and output edges.
        //
        //  One possible estimate is to trace the longest path through the SCC without visiting a node 
        //  more than once (although this is not gaurenteed to be conservative). 

        true
    }

    // Returns sets of nodes invovled in combinational loops.
    fn identify_combinational_loops(&self, tg: &TimingGraph) -> Vec<Vec<NodeId>> {
        // Any SCC of size >=2 is a loop in the timing graph.
        const MIN_LOOP_SCC_SIZE: usize = 2;
        identify_strongly_connected_components(tg, MIN_LOOP_SCC_SIZE)
    }

    /// Return the total number of active edges (if not disabled), unused.
    #[allow(unused)]
    fn count_active_edges(&self, edges_to_check: Range<EdgeId>) -> usize { //usize
        let mut active_cnt: usize = 0;

        for edge in edges_to_check.into_iter() {
            if !self.edge_disabled(&edge) {
                active_cnt += 1;
            }
        }

        active_cnt
    }
}

// Node data accessors (read-only).
impl TimingGraph {
    /// return the node_type by indexing in node types.
    pub fn node_type(&self, id: &NodeId) -> &NodeType {
       &self.node_types[id.clone()]
    }

    pub fn node_out_edges(&self, id: &NodeId) -> Range<EdgeId> {
        Range::new(&self.node_out_edges[id.clone()])
    }

    pub fn node_in_edges(&self, id: &NodeId) -> Range<EdgeId> {
        Range::new(&self.node_in_edges[id.clone()])
    }
    
    pub fn node_num_active_in_edges(&self, id: NodeId) -> usize {
        let mut active_edges: usize = 0;

        for edge_id in self.node_in_edges(&id).into_iter(){
            if self.edge_disabled(&edge_id) == true {
                active_edges = active_edges + 1;
            }
        }

        active_edges
    }

    /// Find the capture edge for the sink node.
    pub fn node_clock_capture_edge(&self, id: NodeId) -> EdgeId {
        match self.node_type(&id) {
            // Only sink nodes can have clock capture edges.
            NodeType::Sink => {
                let mut target_edge_id: EdgeId = EdgeId::new(id.inner()).mark_invalid().clone();

                for edge_id in self.node_in_edges(&id).into_iter() {
                    target_edge_id = match self.edge_type(&edge_id) {
                        EdgeType::PrimitiveClockCapture => edge_id,
                        _ => break,
                    };
                }

                target_edge_id
            },
            _ => EdgeId::new(id.inner()).mark_invalid().clone()
        }
    }

    /// Find the clock launch edge for the source node.
    pub fn node_clock_launch_edge(&self, id: NodeId) -> EdgeId {
        match self.node_type(&id) {
            // Only source nodes can have clock capture edges.
            NodeType::Source => {
                let mut target_edge_id: EdgeId = EdgeId::new(id.inner()).mark_invalid().clone();

                for edge_id in self.node_in_edges(&id).into_iter() {
                    target_edge_id = match self.edge_type(&edge_id) {
                        EdgeType::PrimitiveClockLaunch => edge_id,
                        _ => break,
                    };
                }

                target_edge_id
            },
            _ => EdgeId::new(id.inner()).mark_invalid().clone()
        }
    }

    /// Find the LevelId for the node given.
    pub fn node_level(&self, node_id: NodeId) -> LevelId {
        match self.is_levelized {
            true => self.node_level(node_id),
            false => LevelId::new(node_id.inner()).mark_invalid().clone(),
        }
    }

    /// Return the edge type of the given edge id.
    pub fn edge_type(&self, edge_id: &EdgeId) -> &EdgeType {
        assert!(self.valid_edge_id(&edge_id));
        
        &self.edge_types[edge_id.clone()]
    }

    pub fn edge_sink_node(&self, id: &EdgeId) -> &NodeId {
        &self.edge_sink_nodes[id.clone()]
    }

    pub fn edge_src_node(&self, id: &EdgeId) -> &NodeId {
        &self.edge_src_nodes[id.clone()]
    }

    pub fn edge_disabled(&self, id: &EdgeId) -> bool {
        self.edges_disabled[id.clone()]
    }

    // Find a certain edge by giving source node and sink node.
    // Do not 
    pub fn find_edge(&self, src_node: NodeId, sink_node: NodeId) -> EdgeId {
        assert!(self.valid_node_id(&src_node));
        assert!(self.valid_node_id(&sink_node));

        let mut target_edge = EdgeId::new(0).mark_invalid().clone();
        for edge in self.node_out_edges(&src_node) {
            if *self.edge_sink_node(&edge) == sink_node {
                target_edge = edge;
            }
        }

        target_edge
    }

    pub fn level_nodes(&self, level_id: LevelId) -> Range<NodeId> {
        match self.is_levelized {
            //? No Panic Maybe?
            false => panic!("Timing graph must be levelized!"),
            true => Range::new(&self.level_nodes[level_id])
        }
    }

    pub fn primary_inputs(&self) -> Range<NodeId> {
        match self.is_levelized {
            //? No Panic Maybe?
            false => panic!("Timing graph must be levelized!"),
            true => Range::new(&self.primary_inputs)
        }
    }

    pub fn logical_outputs(&self) -> Range<NodeId> {
        match self.is_levelized {
            false => panic!("Timing graph must be levelized!"),
            true => Range::new(&self.logical_outputs)
        }
    }

    // Return ranges for nodes, edges, levels and reversed levels.

    pub fn nodes(&self) -> Range<NodeId> {
        Range::new_with_lm(&self.node_ids)
    }

    pub fn edges(&self) -> Range<EdgeId> {
        Range::new_with_lm(&self.edge_ids)
    }

    pub fn levels(&self) -> Range<LevelId> {
        match self.is_levelized {
            true => Range::new_with_lm(&self.level_ids),
            false => panic!("Timing graph must be levelized!"),
        }
    }

    pub fn reversed_levels(&self) -> Range<LevelId> {
        match self.is_levelized {
            false => panic!("Timing graph must be levelized!"),
            true => Range::new_with_lmrev(&self.level_ids)
        }
    }

    // The whole validation.
    pub fn validate(&self) -> bool {
        let mut valid = true;

        valid &= self.validate_sizes();

        valid &= self.validate_values();

        valid &= self.validate_structure();

        valid
    }
}

// TimingGraph public modifiers.
impl TimingGraph {
    /// Add a node (automatically increated by 1) by the given NodeType. 
    /// Corresponding edges and node_types will be updated, too.
    pub fn add_node(&mut self, tp: NodeType) -> NodeId {
        // Invalidate the levelization.
        self.is_levelized = false;

        // Add the node_id.
        let node_id = NodeId::new(self.node_ids.size()); // node_id is increased by 1.
        self.node_ids.push_back(node_id.clone());

        // Add node_type.
        self.node_types.push_back(tp);

        // Add edges.
        self.node_out_edges.push_back(vec![]);
        self.node_in_edges.push_back(vec![]);

        // Verify sizes.
        assert_eq!(self.node_types.size(), self.node_out_edges.size());
        assert_eq!(self.node_types.size(), self.node_in_edges.size());

        node_id
    }

    /// Add an edge (automatically increated by 1) by the given NodeType. 
    /// Corresponding edges and node_types will be updated, too.
    pub fn add_edge(&mut self, tp: EdgeType, src_node: NodeId, sink_node: NodeId) -> EdgeId {
        // Require that the source/sink node must already be in the graph,
        //  so we can update them with thier edge references.
        assert!(self.valid_node_id(&src_node));
        assert!(self.valid_node_id(&sink_node));

        //Invalidate the levelization.
        self.is_levelized = false;

        // Reserve the edge ID.
        let edge_id = EdgeId::new(self.edge_ids.size());
        self.edge_ids.push_back(edge_id.clone());

        // Reserve relatives.
        self.edge_types.push_back(tp);
        self.edge_src_nodes.push_back(src_node.clone());
        self.edge_sink_nodes.push_back(sink_node.clone());
        self.edges_disabled.push_back(false);

        // Verify.
        assert_eq!(self.edge_sink_nodes.size(), self.edge_src_nodes.size());

        // Update node edge references.
        self.node_out_edges[src_node].push(edge_id.clone());
        self.node_in_edges[sink_node].push(edge_id.clone());

        edge_id
    }

    /// Remove a node (given node_id) by: 1. mark all node referenced edges as invalid. 
    /// 2. mark node_id in node_ids as invalid.
    pub fn remove_node(&mut self, node_id: NodeId) {
        assert!(self.valid_node_id(&node_id));

        self.is_levelized = false;

        // Invalidate all the references.
        for in_edge in self.node_in_edges(&node_id).into_iter().filter(|x| x.valid()) {
            self.remove_edge(in_edge);
        }

        for out_edge in self.node_in_edges(&node_id).into_iter().filter(|x| x.valid()) {
            self.remove_edge(out_edge);
        }

        self.node_ids[node_id].mark_invalid();
    }

    /// Remove an edge (given edge_id) by: 1. mark all node referenced corresponding edges as invalid. 
    /// 2. mark edge_id in edge_ids as invalid.
    pub fn remove_edge(&mut self, edge_id: EdgeId) {
        assert!(self.valid_edge_id(&edge_id));

        self.is_levelized = false;

        //? Did not implement assert here because no idea what its for.
        let src_node = self.edge_src_node(&edge_id).clone();
        for iter_out in self.node_out_edges[src_node].iter_mut().filter(|x| **x == edge_id) {
            iter_out.mark_invalid();
        }


        let sink_node = self.edge_sink_node(&edge_id).clone();
        for iter_in in self.node_in_edges[sink_node].iter_mut().filter(|x| **x == edge_id) {
            iter_in.mark_invalid();
        }

        self.edge_ids[edge_id].mark_invalid();
    }

    /// Change the disabled status for an edge_id.
    /// Set the levelization as false if we are to change it's status.
    pub fn disable_edge(&mut self, edge_id: EdgeId, disable: bool) {
        assert!(self.valid_edge_id(&edge_id));

        if self.edges_disabled[edge_id.clone()] != disable {
            // If we are changing edges the levelization is no longer valid.
            self.is_levelized = false;
        }

        // Update edges disabled flag.
        self.edges_disabled[edge_id] = disable;
    }

    /// Remap nodes in the TimingGraph. First heck NodeIds are valid or not. 
    /// And make node related attributes or value can be accessed by NodeId as LinearMap Index. 
    /// Also make edge connected Nodes LinearMap correspond to EdgeId
    pub fn remap_nodes(&mut self, nodes_map: &mut LinearMap<NodeId>) {
        self.is_levelized = false;

        // Check the ids are valid or not.
        self.node_ids = clean_and_reorder_ids(nodes_map);
        
        // Reorder NodeId corresponding attributes & values so we can access
        // those values by a given NodeId as Index.
        self.node_types = clean_and_reorder_values(&mut self.node_types, nodes_map);
        self.node_in_edges = clean_and_reorder_values(&mut self.node_in_edges, nodes_map);
        self.node_out_edges = clean_and_reorder_values(&mut self.node_out_edges, nodes_map);

        // Update references.
        self.edge_src_nodes = update_edge_connected_nodes(&self.edge_src_nodes, nodes_map);
        self.edge_sink_nodes = update_edge_connected_nodes(&self.edge_sink_nodes, nodes_map);
    }

    /// Remap edges in the TimingGraph.
    pub fn remap_edges(&mut self, edges_map: &mut LinearMap<EdgeId>) {
        self.is_levelized = false;

        // Check the ids are valid or not.
        self.edge_ids = clean_and_reorder_ids(edges_map);

        self.edge_types = clean_and_reorder_values(&mut self.edge_types, edges_map);
        self.edge_sink_nodes = clean_and_reorder_values(&mut self.edge_sink_nodes, edges_map);
        self.edge_src_nodes = clean_and_reorder_values(&mut self.edge_src_nodes, edges_map);
        self.edges_disabled = clean_and_reorder_values(& mut self.edges_disabled, edges_map);

        for edges_ref in self.node_in_edges.iter_mut() {
            *edges_ref = update_node_referenced_edges(edges_ref, edges_map);
        }

        for edges_ref in self.node_out_edges.iter_mut() {
            *edges_ref = update_node_referenced_edges(edges_ref, edges_map);
        }
    }

    //TODO: Finish this one.
    pub fn compress(&mut self) -> GraphIdMaps {
        let mut node_id_map = compress_ids(&self.node_ids);
        let mut edge_id_map = compress_ids(&self.edge_ids);

        self.remap_nodes(&mut node_id_map);
        self.remap_edges(&mut edge_id_map);

        self.levelize();
        self.validate();

        GraphIdMaps {
            node_id_map,
            edge_id_map,
        }
    }

    /// Levelize the graph
    pub fn levelize(&self) {
        if !self.is_levelized {
            self.force_levelize(); //TODO: Implement.
        }
    }

    /// Memory layout optimization operations.
    /// Returns GraphIdMaps.
    pub fn optimize_layout(&mut self) -> GraphIdMaps {
        let mut nodes_map = self.optimize_node_layout();
        self.remap_nodes(&mut nodes_map);

        self.levelize();

        let mut edges_map = self.optimize_edge_layout();
        self.remap_edges(&mut edges_map);

        self.levelize();

        GraphIdMaps {
            node_id_map: nodes_map,
            edge_id_map: edges_map,
        }
    }

    /// Sets whether dangling combinational nodes is an error (if true) or not.
    pub fn set_allow_dangling_combinational_nodes(&mut self, value: bool) {
        self.allow_dangling_combinational_nodes = value;
    }
}

/// Mappings from old to new IDs.
#[allow(unused)]
pub struct GraphIdMaps {
    node_id_map: LinearMap<NodeId>,
    edge_id_map: LinearMap<EdgeId>,
}

impl GraphIdMaps {
    /// Create a GraphIdMaps from NodeId and EdgeId LearnMap. 
    /// In fact, it just put 2 LinearMaps into it.
    pub fn new(node_map: LinearMap<NodeId>, edge_map: LinearMap<EdgeId>) -> Self {
        GraphIdMaps {
            node_id_map: node_map,
            edge_id_map: edge_map,
        }
    }
}
