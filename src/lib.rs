
use std::fmt;
use std::result;
use std::collections::VecDeque;

pub type Result = result::Result<(), String>;

/// A unique identifier for a vertex within a graph
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VertexId { value: usize }

/// A directed edge from a Vertex pointing to another
#[derive(Debug, Copy, Clone)]
pub struct Arc {
    other: VertexId,
    weight: u64
}

/// A vertex in a graph
pub struct Vertex<A> {
    value: A,
    arcs: Vec<Arc>,
    id: VertexId
}

/// A directed graph
pub struct DirectedGraph<A> {
    vertices: Vec<Vertex<A>>
}

impl <A> DirectedGraph<A> {

    /// Constructs a new empty directed graph
    pub fn new() -> DirectedGraph<A> {
        DirectedGraph { vertices: Vec::new() }
    }

    /// Retrieves the vertex at the given id
    pub fn vertex(&self, id: VertexId) -> Option<&Vertex<A>> {
        self.vertices.get(id.value)
    }

    /// Retrieves the vertex at the given id
    pub fn vertex_mut(&mut self, id: VertexId) -> Option<&mut Vertex<A>> {
        self.vertices.get_mut(id.value)
    }

    /// Retrieves a vertex value
    pub fn vertex_value(&self, id: VertexId) -> Option<&A> {
        self.vertex(id).map(|vertex| &vertex.value)
    }

    /// Retrieves a vertex value
    pub fn vertex_value_mut(&mut self, id: VertexId) -> Option<&mut A> {
        self.vertex_mut(id).map(|vertex| &mut vertex.value)
    }

    /// Retrieves the vertex value from the graph
    pub fn add_vertex(&mut self, value: A) -> VertexId {
        let id = VertexId { value: self.vertices.len() };
        self.vertices.push(Vertex { value: value, arcs: Vec::new(), id: id });
        id
    }

    /// Connects two vertices
    pub fn connect(&mut self, from: VertexId, to: VertexId, weight: u64) -> Result {
        match self.vertices.get_mut(from.value) {
            Some(vertex) => {
                let arc = Arc { other: to, weight: weight };
                vertex.arcs.push(arc);
                Ok(())
            },
            None => Err(format!("{:?} does not exist", from))
        }
    }

    /// Iterate over values in breadth-first order
    pub fn breadth_first_iter(&self, from: VertexId) -> BFDirectedGraphIterator<A> {
        let mut visited = Vec::new();
        let mut q = VecDeque::new();

        visited.resize(self.vertices.len(), false);

        q.push_back(Arc { other: from, weight: 0 });

        BFDirectedGraphIterator {
            graph: self,
            visited: visited,
            q: q
        }
    }

    /// Iterate over values in depth-first order
    pub fn depth_first_values_iter(&self, from: VertexId) -> DFDirectedGraphValueIterator<A> {
        DFDirectedGraphValueIterator { iter: self.depth_first_iter(from) }
    }

    pub fn depth_first_iter(&self, from: VertexId) -> DFDirectedGraphIterator<A> {

        let mut visited = Vec::new();
        let mut stack = Vec::new();

        visited.resize(self.vertices.len(), false);

        stack.push(Arc { other: from, weight: 0 });

        DFDirectedGraphIterator {
            graph: self,
            visited: visited,
            stack: stack
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Checks if the graph is cyclic
    fn is_cyclic(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            let head = &self.vertices[0];
            for vertex in self.depth_first_iter(head.id) {
                for arc in &vertex.arcs {
                    let other_vertex = &self.vertices[arc.other.value];
                    for reverse_arc in &other_vertex.arcs {
                        if reverse_arc.other == vertex.id {
                            return true
                        }
                    }
                }
            }
            return false;
        }
    }

    /// The out-degree of a vertex
    pub fn out_degree(&self, vertex_id: VertexId) -> usize {
        self.vertices[vertex_id.value].arcs.len()
    }

    /// The in-degree of a vertex
    pub fn in_degree(&self, vertex_id: VertexId) -> usize {
        self.vertices
            .iter()
            .flat_map(|vertex| vertex.arcs.iter())
            .filter(|arc| arc.other == vertex_id)
            .count()
    }
}

/// Breadth-first Graph Iterator
pub struct BFDirectedGraphIterator<'a, A : 'a> {
    graph: &'a DirectedGraph<A>,
    visited: Vec<bool>,
    q: VecDeque<Arc>
}

impl <'a, A> Iterator for BFDirectedGraphIterator<'a, A> {
    type Item = &'a A;
    fn next(&mut self) -> Option<&'a A> {
        match self.q.pop_front() {
            Some(arc) if self.visited[arc.other.value] => {
                self.next()
            },
            Some(arc) => {
                let vertex = &self.graph.vertices[arc.other.value];
                let mut sorted_arcs = vertex.arcs.clone();
                sorted_arcs.sort_unstable_by_key(|arc| arc.weight);
                for arc in sorted_arcs {
                    self.q.push_back(arc);
                }
                self.visited[arc.other.value] = true;
                Some(&vertex.value)
            },
            _ => None
        }
    }
}

/// Depth-first Graph Iterator
pub struct DFDirectedGraphIterator<'a, A : 'a> {
    graph: &'a DirectedGraph<A>,
    visited: Vec<bool>,
    stack: Vec<Arc>
}

impl <'a, A> Iterator for DFDirectedGraphIterator<'a, A> {
    type Item = &'a Vertex<A>;
    fn next(&mut self) -> Option<&'a Vertex<A>> {
        match self.stack.pop() {
            Some(arc) if self.visited[arc.other.value] => {
                self.next()
            },
            Some(arc) => {
                let vertex = &self.graph.vertices[arc.other.value];
                let mut sorted_arcs = vertex.arcs.clone();
                sorted_arcs.sort_unstable_by_key(|arc| arc.weight);
                for arc in sorted_arcs {
                    self.stack.push(arc);
                }
                self.visited[arc.other.value] = true;
                Some(&vertex)
            },
            _ => None
        }
    }
}

/// Depth-first Graph Iterator
pub struct DFDirectedGraphValueIterator<'a, A : 'a> {
    iter: DFDirectedGraphIterator<'a, A>
}

impl <'a, A> Iterator for DFDirectedGraphValueIterator<'a, A> {
    type Item = &'a A;
    fn next(&mut self) -> Option<&'a A> {
        self.iter.next().map(|vertex| &vertex.value)
    }
}

impl <A : fmt::Display> fmt::Display for DirectedGraph<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Graph of {} vertices:", self.vertices.len());
        for vertex in self.vertices.iter() {
            for arc in vertex.arcs.iter() {
                let _ = writeln!(f, "\t ({}:{}) -(weight: {})-> ({}:{})",
                                 vertex.id,
                                 self.vertex_value(vertex.id).expect("Failed to get vertex value"),
                                 arc.weight,
                                 arc.other,
                                 self.vertex_value(arc.other).expect("Failed to get vertex value"));
            }
        }
        write!(f, "")
    }
}

impl fmt::Display for VertexId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VertexId({})", self.value)
    }
}

/// An undirected graph
pub struct UndirectedGraph<A> {
    directed: DirectedGraph<A>
}

impl <A> UndirectedGraph<A> {

    /// Connects two vertices bidirectionally
    pub fn connect_undirected(&mut self, one: VertexId, other: VertexId, weight: u64) -> Result {
        match self.directed.connect(one, other, weight) {
            Ok(()) => self.directed.connect(other, one, weight),
            err => err
        }
    }

    pub fn new() -> UndirectedGraph<A> {
        UndirectedGraph { directed: DirectedGraph::new() }
    }

    /// Retrieves a vertex value
    pub fn vertex_value(&self, id: VertexId) -> Option<&A> {
        self.directed.vertex_value(id)
    }

    /// Retrieves a vertex value
    pub fn vertex_value_mut(&mut self, id: VertexId) -> Option<&mut A> {
        self.directed.vertex_value_mut(id)
    }

    /// Retrieves the vertex value from the graph
    pub fn add_vertex(&mut self, value: A) -> VertexId {
        self.directed.add_vertex(value)
    }

    pub fn is_empty(&self) -> bool {
        self.directed.is_empty()
    }
}

impl <A: fmt::Display> fmt::Display for UndirectedGraph<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.directed)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bf_iter() {
        let mut graph = DirectedGraph::new();

        let zero = graph.add_vertex("zero".to_string());
        let one = graph.add_vertex("one".to_string());
        let two = graph.add_vertex("two".to_string());
        let three = graph.add_vertex("three".to_string());

        let _ = graph.connect(zero, one, 1);
        let _ = graph.connect(zero, two, 1);
        let _ = graph.connect(one, two, 1);
        let _ = graph.connect(two, zero, 1);
        let _ = graph.connect(two, three, 1);
        let _ = graph.connect(three, three, 1);

        assert_eq!(
            graph.breadth_first_iter(two).collect::<Vec<&String>>(),
            vec!["two", "zero", "three", "one"]
        )
    }

    #[test]
    fn df_iter() {
        let mut graph = DirectedGraph::new();

        let zero = graph.add_vertex("zero".to_string());
        let one = graph.add_vertex("one".to_string());
        let two = graph.add_vertex("two".to_string());
        let three = graph.add_vertex("three".to_string());

        let _ = graph.connect(zero, one, 0);
        let _ = graph.connect(zero, two, 0);
        let _ = graph.connect(one, two, 0);
        let _ = graph.connect(two, zero, 1);
        let _ = graph.connect(two, three, 0);
        let _ = graph.connect(three, three, 0);

        assert_eq!(
            graph.depth_first_values_iter(two).collect::<Vec<&String>>(),
            vec!["two", "zero", "one", "three"]
        )
    }

    #[test]
    fn is_empty() {

        let mut graph = DirectedGraph::new();

        assert!(graph.is_empty());

        let _ = graph.add_vertex("three".to_string());

        assert!(!graph.is_empty())
    }

    #[test]
    fn is_cyclic() {

        let mut graph = DirectedGraph::new();

        let zero = graph.add_vertex("zero".to_string());
        let one = graph.add_vertex("one".to_string());
        let two = graph.add_vertex("two".to_string());
        let three = graph.add_vertex("three".to_string());

        let _ = graph.connect(zero, one, 0);
        let _ = graph.connect(zero, two, 0);
        let _ = graph.connect(one, two, 0);
        let _ = graph.connect(two, three, 0);

        assert!(!graph.is_cyclic());

        let _ = graph.connect(two, zero, 0);

        assert!(graph.is_cyclic())
    }

    #[test]
    fn out_and_in_degrees() {

        let mut graph = DirectedGraph::new();

        let zero = graph.add_vertex("zero".to_string());
        let one = graph.add_vertex("one".to_string());
        let two = graph.add_vertex("two".to_string());
        let three = graph.add_vertex("three".to_string());

        let _ = graph.connect(zero, one, 0);
        let _ = graph.connect(zero, two, 0);
        let _ = graph.connect(one, two, 0);
        let _ = graph.connect(two, zero, 1);
        let _ = graph.connect(two, three, 0);
        let _ = graph.connect(three, three, 0);

        assert_eq!(graph.in_degree(two), 2);
        assert_eq!(graph.out_degree(two), 2);
        assert_eq!(graph.in_degree(one), 1);
        assert_eq!(graph.out_degree(one), 1);
        assert_eq!(graph.in_degree(three), 2);
        assert_eq!(graph.out_degree(three), 1);
    }
}
