use std::{ops::{Index, IndexMut}, hash::{Hash}, collections::{VecDeque, btree_map::Iter}, vec, usize, array::IntoIter};

use super::graph::TimingGraph;

// TODO: Keep basic data types and helpful data structs here, and move helpful functions out.
// TODO: I'm solve the multi-reference-then-move problem by using .clone(). Check for more efficient way.
/// Define basic data types.

/// Timing analysis arrival time: Eearly, Late.
pub enum ArrivalType {
    Eearly,
    Late,
}

/// Timing analysis delay type: Max, Min.
pub enum DelayType {
    Max,
    Min,
}

/// Timing analysis type: Setup, Hold, Unknown.
pub enum TimingType {
    Setup,
    Hold,
    Unknown,
}

/// Potential types for nodes in the timing graph:
/// Source: The start of a clock/data path
/// Sink: The end of a clock/data path
/// Ipin: An intermediate input pin
/// Opin: An intermediate output pin
/// Cpin: An intermediate clock (input) pin
/// NodeFiller: Meanless filler only for continuous linear map
#[derive(Clone)]
pub enum NodeType {
    Source,
    Sink,
    Ipin,
    Opin,
    Cpin,
    NodeFiller,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::NodeFiller
    }
}

/// Edge types.
#[derive(Clone)]
pub enum EdgeType {
    PrimitiveCombinational,
    PrimitiveClockLaunch,
    PrimitiveClockCapture,
    InterConnect,
    EdgeFiller,
}

impl Default for EdgeType {
    fn default() -> Self {
        EdgeType::EdgeFiller
    }
}

// Index for checking duplicates edges (in HashMap).
#[derive(Hash)]
pub struct Pair<'a> {
    pub src_node: &'a NodeId,
    pub sink_node: &'a NodeId,
}

impl<'a> PartialEq for Pair<'a> {
    // Compare the sink & source nodes are the same by comparing their id.
    fn eq(&self, other: &Self) -> bool {
        self.src_node.inner() == other.src_node.inner() && self.sink_node.inner() == other.sink_node.inner()
    }
}

impl<'a> Eq for Pair<'a> {}

/// Validable ids, contains primitive value and a tag for checking whether it's valid
/// Allow to access usize inner value by calling .inner().
pub trait Validable {
    fn new(id: usize) -> Self;
    fn inner(&self) -> usize;
    fn valid(&self) -> bool;

    // Mark the Id as valid and return the Id by Clone.
    fn mark_valid(&mut self) -> &Self;

    // Mark the Id as invalid and return the id by Clone.
    fn mark_invalid(&mut self) -> &Self;
}

/// StrongId implemented in rust newtype.
#[derive(Clone, Debug, Hash, Default)]
pub struct StrongId {
    primitive_val: usize,
    valid: bool,
}

/// StrongId implemented in rust newtype.
/// If default is used, the NodeId is invalid with 'valid: false'.
#[derive(Clone, Debug, Hash, Default)]
pub struct NodeId(StrongId);

impl Validable for NodeId {
    fn new(id: usize) -> Self {
        NodeId(StrongId {
            primitive_val: id,
            valid: true,
        })

    }

    fn inner(&self) -> usize {
        self.0.primitive_val
    }

    fn valid(&self) -> bool {
        self.0.valid
    }

    fn mark_valid(&mut self) -> &Self {
        self.0.valid = true;
        self
    }

    fn mark_invalid(&mut self) -> &Self {
        self.0.valid = false;
        self
    }
}

impl PartialEq for NodeId {
    fn eq(&self, other:&Self) -> bool {
        self.inner() == other.inner()
    }
}

impl Eq for NodeId {}

impl PartialOrd for NodeId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner().cmp(&other.inner())
    }
}

/// StrongId implemented in rust newtype.
#[derive(Clone, Debug, Hash, Default)]
pub struct BlkId(StrongId);

impl Validable for BlkId {
    fn new(id: usize) -> Self {
        BlkId(StrongId {
            primitive_val: id,
            valid: true,
        })
    }

    fn inner(&self) -> usize {
        self.0.primitive_val
    }

    fn valid(&self) -> bool {
        self.0.valid
    }

    fn mark_valid(&mut self) -> &Self {
        self.0.valid = true;
        self
    }

    fn mark_invalid(&mut self) -> &Self {
        self.0.valid = false;
        self
    }
}

impl PartialEq for BlkId {
    fn eq(&self, other:&Self) -> bool {
        self.inner() == other.inner()
    }
}

/// StrongId implemented in rust newtype.
#[derive(Clone, Debug, Hash, Default)]
pub struct EdgeId(StrongId);

impl Validable for EdgeId {
    fn new(id: usize) -> Self {
        EdgeId(StrongId {
            primitive_val: id,
            valid: true,
        })
    }

    fn inner(&self) -> usize {
        self.0.primitive_val
    }

    fn valid(&self) -> bool {
        self.0.valid
    }

    fn mark_valid(&mut self) -> &Self {
        self.0.valid = true;
        self
    }

    fn mark_invalid(&mut self) -> &Self {
        self.0.valid = false;
        self
    }
}

impl PartialEq for EdgeId {
    fn eq(&self, other:&Self) -> bool {
        self.inner() == other.inner()
    }
}

/// StrongId implemented in rust newtype.
#[derive(Clone, Debug, Hash, Default)]
pub struct LevelId(StrongId);

impl Validable for LevelId {
    fn new(id: usize) -> Self {
        LevelId(StrongId {
            primitive_val: id,
            valid: true,
        })
    }

    fn inner(&self) -> usize {
        self.0.primitive_val
    }

    fn valid(&self) -> bool {
        self.0.valid
    }

    fn mark_valid(&mut self) -> &Self {
        self.0.valid = true;
        self
    }

    fn mark_invalid(&mut self) -> &Self {
        self.0.valid = false;
        self
    }
}

impl PartialEq for LevelId {
    fn eq(&self, other:&Self) -> bool {
        self.inner() == other.inner()
    }
}

/// StrongId implemented in rust newtype.
#[derive(Clone, Debug, Hash, Default)]
pub struct DomainId(StrongId);

impl Validable for DomainId {
    fn new(id: usize) -> Self {
        DomainId(StrongId {
            primitive_val: id,
            valid: true,
        })
    }

    fn inner(&self) -> usize {
        self.0.primitive_val
    }

    fn valid(&self) -> bool {
        self.0.valid
    }

    fn mark_valid(&mut self) -> &Self {
        self.0.valid = true;
        self
    }

    fn mark_invalid(&mut self) -> &Self {
        self.0.valid = false;
        self
    }
}

impl PartialEq for DomainId {
    fn eq(&self, other:&Self) -> bool {
        self.inner() == other.inner()
    }
}

/// LinearMap is the basic data type to hold a series of data, like edges, nodes, levels and 
/// their corresponding types. The data inside LinearMap is just a vector, but can be indexed by
/// strongid types. For implementations, we ask K to be Validable because we need the primitive value
/// as index, but we ask V to be both Clone and Validable.
pub struct LinearMap<V: Clone> {
    vec_: Vec<V>,
}

/// Overloading Index trait. Here K must be a StrongId type and can be 
/// converted into usize for comparing and indexing.
impl<K: Validable, V: Clone> Index<K> for LinearMap<V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        assert!(index.inner() < self.vec_.len(), "Out-of-range index");
        &self.vec_[index.inner()]
    }
}

impl<K: Validable, V: Clone> IndexMut<K> for LinearMap<V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        assert!(index.inner() < self.vec_.len(), "Out-of-range index");
        &mut self.vec_[index.inner()]
    }
}

/// Linear_Map read_only accessors.
impl<V: Clone> LinearMap<V> {
    // Returns an iterator for element reference.
    pub fn l_iter(&self) -> impl Iterator<Item = &V> {
        self.vec_.iter()
    }

    // Returns an reversed_iterator.
    pub fn r_iter(&self) -> impl Iterator<Item = &V> {
        self.vec_.iter().rev()
    }

    // If those functions need to be called repeatedly, add them to properties.
    pub fn begin(&self) -> Option<&V> {
        self.l_iter().next()
    }

    pub fn end(&self) -> Option<&V> {
        self.l_iter().last()
    }

    pub fn rbegin(&self) -> Option<&V> {
        self.r_iter().next()
    }

    pub fn rend(&self) -> Option<&V> {
        self.r_iter().last()
    }

    // Read vector properties.
    pub fn empty(&self) -> bool {
        self.vec_.is_empty()
    }

    pub fn size(&self) -> usize {
        self.vec_.len()
    }

    pub fn capacity(&self) -> usize {
        self.vec_.capacity()
    }

    pub fn vec(&self) -> Vec<V> {
        self.l_iter().map(|x| x.clone()).collect()
    }

    pub fn vec_rev(&self) -> Vec<V> {
        self.r_iter().map(|x| x.clone()).collect()
    }

    // Those function are not effectively used.
    // fn find()
    // fn contain()
}

// LinearMap constructors.
impl<V: Clone> LinearMap<V>{
    pub fn new_with_vector(values: Vec<V>) -> LinearMap<V> {
        LinearMap {
            vec_: values,
        }
    }

    /// Calling this method will possibly reduce the capacity for the newly 
    /// created LinearMap because in rust .len() and .capacity() are different
    /// and here we use .len() from the argument to create a new LinearMap.
    pub fn new_with_capacity_by_lm(values: &LinearMap<V>) -> LinearMap<V> {
        LinearMap {
            vec_: Vec::with_capacity(values.size()),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> LinearMap<V> {
        LinearMap {
            vec_: Vec::with_capacity(capacity),
        }
    }

    /// New with the given size and filled with initial values for indexing.
    pub fn new_with_size(size: usize, value: V) -> LinearMap<V> {
        let mut lm = LinearMap {
            vec_: Vec::with_capacity(size),
        };

        // Resize and fill the initial value.
        lm.resize_and_fill_default(lm.capacity(), value);

        lm
    }
}

// LinearMap Mutators.
impl<V: Clone> LinearMap<V> {
    /// Returns a mutable into_iter.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.vec_.iter_mut()
    }

    /// Push one value at the back of the vector.
    pub fn push_back (&mut self, value: V) -> () {
        self.vec_.push(value);
    }

    /// Extend a collection at the back of the vector.
    pub fn push_back_collection(&mut self, values: Vec<V>) -> () {
        self.vec_.extend(values);
    }

    // Replace the resize() in original code.
    /// Only change the capacity of the vector.
    pub fn reserve(&mut self, len_more: usize) -> () {
        self.vec_.reserve(len_more);
    }

    /// Change the capacity and resize the LinearMap full with initial value.
    pub fn reserve_and_fill_default(&mut self, len_more: usize, value: V) -> () {
        self.vec_.reserve(len_more);
        self.vec_.resize(self.capacity(), value);
    }

    // Resize and fill defaults without change the capacity.
    pub fn resize_and_fill_default(&mut self, len: usize, value: V) -> () {
        self.vec_.resize(self.capacity(), value);
    }

    // Change the size and append new value.
    // If resize len + 1 and then add a value to the vector is more efficient than push().
    // Then we need to implement this method.
    // Otherwise, just use push().
    // pub fn resize(&mut self, len: usize, value: V) -> () {
    //     self.vec_.resize(len, value)
    // }

    /// Shrink to just fit the size
    pub fn shrink_to_fit(&mut self) -> () {
        self.vec_.shrink_to_fit();
    }

    /// Clear the vector.
    pub fn clear(&mut self) -> () {
        self.vec_.clear();
    }

    // Ignore emplace_back: no such thing in rust implementation.
}

/// Inconsistent implementation than the original code because we fill all the extended capacity
/// with invalid StrongId. And we use .clone() instead of move to insert value which could be less
/// effiecient. However, in this way we can keep the capacity always same as length.
impl<V: Clone + Default> LinearMap<V> {
    /// Allow to insert the value in an incontinuous way (not recommanded!).
    pub fn insert<K: Validable>(&mut self, key: &K, value: &mut V) -> () {
        // K represents a StrongId type which contains primitive value.
        // Insert value in the index (primitive value of K) of the vector.
        if key.inner() >= self.vec_.len() {
            // Rust vector can only index by len, not capacity.
            // So we fill it with invalid values for incontinuous data.
            // This could be less efficient 
            self.vec_.resize(key.inner() + 1, Default::default()); //TODO: Find ideal filler.
        }

 
        // Won't work to index capacity.
        // std::mem::replace(&mut self.vec_[key.inner()], value);
        self.vec_[key.inner()] = value.clone()
    }
}

// More LinearMap Options.
pub fn swap<K: Validable, V: Clone>(mut x: LinearMap<V>, mut y: LinearMap<V>) -> () {
    std::mem::swap(&mut x.vec_, &mut y.vec_)
}

// Range object.

/// Range will also hold vector, the main difference is that it exposes less methods and associated functions. 
/// Range have to be created(new) then to access it's members, and it's for read-only!
pub struct Range<'a, V:Clone> {
    vec_: Vec<V>,
    begin_: Option<&'a V>,
    end_: Option<&'a V>,
}

impl<'a, V: Clone> Range<'a, V> {
    pub fn new(values: &'a Vec<V>) -> Self {
        Range {
            vec_: values.clone(),
            begin_: values.iter().next(),
            end_: values.iter().last(),
        }
    }

    pub fn new_with_lm(values: &'a LinearMap<V>) -> Self {
        Range {
            //? Performance Waste
            vec_: values.vec(),
            begin_: values.begin(),
            end_: values.end(),
        }
    }

    pub fn new_with_lmrev(values: &'a LinearMap<V>) -> Self {
        Range {
            vec_: values.vec_rev(),
            begin_: values.begin(),
            end_: values.end(),
        }
    }

    pub fn begin(&self) -> Option<&V> {
        self.begin_
    }

    pub fn end(&self) -> Option<&V> {
        self.end_
    }

    pub fn empty(&self) -> bool {
        self.vec_.is_empty()
    }

    pub fn size(&self) -> usize {
        self.vec_.len()
    }
}

// Call Range.into_iter() to iterate the inside vec_.
impl<'a, V: Clone> IntoIterator for Range<'a, V> {
    type Item = V;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec_.into_iter()
    }
}

/// Basic Node information handler?
#[derive(Clone)]
pub struct NodeNccInfo {
    pub on_stack: bool, // Whether the node has been pushed to stack.
    pub index: isize, // Index for this node.
    pub low_link: isize, // Lowest index for the nodes connected to this one and itself.
}

// loop detectors.

/// Returns sets of nodes (strongly connected components) which exceed the specigied min_size.
pub fn identify_strongly_connected_components(tg: &TimingGraph, min_size: usize) -> Vec<Vec<NodeId>>{
    //This uses Tarjan's algorithm which identifies Strongly Connected Components (SCCs) in O(|V| + |E|) time.
    let mut curr_index: isize = 0;
    let mut stack: VecDeque<NodeId> = VecDeque::new(); // FILO for storing NodeId.
    
    // node_info: LinearMap whose key is NodeId and its value is NodeNccInfo.
    // By default all nodes are not strongly connected (index: -1, low_link: -1).
    let mut node_info = LinearMap::<NodeNccInfo>::new_with_size(tg.nodes().size(), NodeNccInfo { on_stack: false, index: -1, low_link: -1 });
    let mut sccs: Vec<Vec<NodeId>> = vec![];

    // Build strongly connected nodes.
    for node_id in tg.nodes().into_iter() {
        match node_info[node_id.clone()].index {
            -1 => strongconnect(tg, node_id, &mut curr_index, &mut stack, &mut node_info, &mut sccs, min_size),
            _ => (),
        }
    }

    sccs
}

pub fn strongconnect(tg: &TimingGraph, node_id: NodeId, curr_index: &mut isize, stack: &mut VecDeque<NodeId>, node_info: &mut LinearMap<NodeNccInfo>, sccs: &mut Vec<Vec<NodeId>>, min_size: usize) {
    // Change the index and low_link to current index.
    node_info[node_id.clone()].index = *curr_index;
    node_info[node_id.clone()].low_link = *curr_index;

    *curr_index += 1;

    // Stack the node if it has been visited.
    stack.push_back(node_id.clone());
    node_info[node_id.clone()].on_stack = true;

    // Check sink nodes (by finding out edges first)
    for edge in tg.node_out_edges(&node_id) {
        if tg.edge_disabled(&edge) {
            continue;
        }

        // Find the sink node of the node_id, by tracing the out_edges of that node_id.
        let sink_node = tg.edge_sink_node(&edge);

        match node_info[sink_node.clone()].on_stack {
            // Change the lowest linked index according to the minimal node index.
            true => node_info[node_id.clone()].low_link = std::cmp::min(node_info[node_id.clone()].index, node_info[sink_node.clone()].index),
            // If the sink node is not visited, pay visit and change the lowest linked index according to the minimal node index.
            false => {
                strongconnect(tg, node_id.clone(), curr_index, stack, node_info, sccs, min_size);
                node_info[node_id.clone()].low_link = std::cmp::min(node_info[node_id.clone()].index, node_info[sink_node.clone()].index);
            },
        }
    }

    // If this node is the root of a set of strongly connected nodes.
    if node_info[node_id.clone()].low_link == node_info[node_id.clone()].index {
        let mut scc: Vec<NodeId> = vec![];

        loop {
            // Initialize scc_node with popped node.
            let scc_node = stack.pop_back().unwrap();

            // Change scc_node on_stack status.
            node_info[scc_node.clone()].on_stack = false;

            // Storage this node as strongly connected.
            scc.push(scc_node.clone());

            // Break if the popped node it not the node_id currently under visit.
            if scc_node != node_id {
                break;
            }
        }

        // If reach or exceed the min_size restriction for SCC. Add it to final collections.
        if scc.len() >= min_size {
            sccs.push(scc);
        }
    }
}