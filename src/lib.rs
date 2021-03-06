
use std::fmt;
use std::result;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::i64;

pub type Result = result::Result<(), String>;

/// A unique identifier for a vertex within a graph
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VertexId { value: usize }

/// A directed edge from a Vertex pointing to another
#[derive(Debug, Copy, Clone)]
pub struct Arc {
    other: VertexId,
    weight: i64
}

/// A vertex in a graph
pub struct Vertex<A> {
    value: A,
    arcs_in: Vec<Arc>,
    arcs_out: Vec<Arc>,
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
        self.vertices.push(Vertex { value: value, arcs_out: Vec::new(), arcs_in: Vec::new(), id });
        id
    }

    /// Connects two vertices
    pub fn connect(&mut self, from: VertexId, to: VertexId, weight: i64) -> Result {
        self.vertex_mut(from).ok_or(format!("{:?} does not exist", from))?.arcs_out.push(Arc { other: to, weight });
        self.vertex_mut(to).ok_or(format!("{:?} does not exist", from))?.arcs_in.push(Arc { other: from, weight });
        Ok(())
    }

    /// Iterate over vertices in breadth-first order
    pub fn breadth_first_iter(&self, from: VertexId) -> BFDirectedGraphIter<A> {
        let mut visited = Vec::new();
        let mut q = VecDeque::new();

        visited.resize(self.vertices.len(), false);

        q.push_back(Arc { other: from, weight: 0 });

        BFDirectedGraphIter {
            graph: self,
            visited: visited,
            q: q
        }
    }

    pub fn depth_first_iter(&self, from: VertexId) -> DFDirectedGraphIter<A> {

        let mut visited = Vec::new();
        let mut stack = Vec::new();

        visited.resize(self.vertices.len(), false);

        stack.push(Arc { other: from, weight: 0 });

        DFDirectedGraphIter {
            graph: self,
            visited: visited,
            stack: stack
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Checks if the graph is cyclic
    pub fn is_cyclic(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            let head = &self.vertices[0];
            for vertex in self.depth_first_iter(head.id) {
                for arc in &vertex.arcs_out {
                    let other_vertex = &self.vertices[arc.other.value];
                    for reverse_arc in &other_vertex.arcs_out {
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
    pub fn out_degree(&self, vertex_id: VertexId) -> Option<usize> {
        self.vertex(vertex_id).map(|vertex| vertex.arcs_out.len())
    }

    /// The in-degree of a vertex
    pub fn in_degree(&self, vertex_id: VertexId) -> Option<usize> {
        self.vertex(vertex_id).map(|vertex| vertex.arcs_in.len())
    }

    /// Depth-first-search
    fn dfs(&self, from: VertexId, visited: &mut HashSet<VertexId>,
           pre_children_visit: &mut FnMut(VertexId) -> (), post_children_visit: &mut FnMut(VertexId) -> ()) -> Result {
        if visited.contains(&from) { return Ok(()) }
        visited.insert(from);
        match self.vertex(from) {
            None => Err(format!("{:?} could not be found", from)),
            Some(from_vertex) => {
                pre_children_visit(from);
                for arc_out in &from_vertex.arcs_out {
                    let result = self.dfs(arc_out.other, visited, pre_children_visit, post_children_visit);
                    if result.is_err() {
                        return result;
                    }
                }
                post_children_visit(from);
                Ok(())
            }
        }
    }

    fn topological_order(&self) -> Vec<VertexId> {
        let mut last_counter = self.vertices.len();
        let mut order = Vec::new();
        order.resize(self.vertices.len(), VertexId { value: 0 });
        let mut visited = HashSet::with_capacity(self.vertices.len());

        for vertex in &self.vertices {
            let result = self.dfs(
                vertex.id,
                &mut visited,
                &mut |_| (),
                &mut |vertex_id| { order[last_counter-1] = vertex_id; last_counter -= 1; }
            );

            if result.is_err() {
                panic!(result)
            }
        }
        order
    }

    /// Returns an iterator over topologically-ordered vertices if the graph is acyclic
    pub fn topologically_ordered_iter(&self) -> Option<TopologicalIter<A>> {
        if self.is_cyclic() {
            None
        } else {
            let mut order = self.topological_order();
            order.reverse();
            Some(TopologicalIter { graph: self, order })
        }
    }

    /// Returns the longest distance from the source to each other reachable vertex
    pub fn longest_distance_from(&self, source: VertexId) -> Option<HashMap<VertexId, i64>> {
        let mut distances = HashMap::new();
        distances.insert(source, 0);
        match self.topologically_ordered_iter() {
            None => None,
            Some(iter) => {
                for vertex in iter {
                    for arc in &vertex.arcs_out {
                        let other_distance = distances.get(&vertex.id).unwrap_or(&i64::MIN) + &arc.weight;
                        if distances.get(&arc.other).unwrap_or(&i64::MIN) < &other_distance {
                            distances.insert(arc.other, other_distance);
                        }
                    }
                }
                Some(distances)
            }
        }
    }
}

/// Breadth-first Graph Iterator
pub struct BFDirectedGraphIter<'a, A : 'a> {
    graph: &'a DirectedGraph<A>,
    visited: Vec<bool>,
    q: VecDeque<Arc>
}

impl <'a, A> Iterator for BFDirectedGraphIter<'a, A> {
    type Item = &'a Vertex<A>;
    fn next(&mut self) -> Option<&'a Vertex<A>> {
        match self.q.pop_front() {
            Some(arc) if self.visited[arc.other.value] => {
                self.next()
            },
            Some(arc) => {
                let vertex = &self.graph.vertices[arc.other.value];
                let mut sorted_arcs = vertex.arcs_out.clone();
                sorted_arcs.sort_unstable_by_key(|arc| arc.weight);
                for arc in sorted_arcs {
                    self.q.push_back(arc);
                }
                self.visited[arc.other.value] = true;
                Some(&vertex)
            },
            _ => None
        }
    }
}

/// Depth-first Graph Iterator
pub struct DFDirectedGraphIter<'a, A : 'a> {
    graph: &'a DirectedGraph<A>,
    visited: Vec<bool>,
    stack: Vec<Arc>
}

impl <'a, A> Iterator for DFDirectedGraphIter<'a, A> {
    type Item = &'a Vertex<A>;
    fn next(&mut self) -> Option<&'a Vertex<A>> {
        match self.stack.pop() {
            Some(arc) if self.visited[arc.other.value] => {
                self.next()
            },
            Some(arc) => {
                let vertex = &self.graph.vertices[arc.other.value];
                let mut sorted_arcs = vertex.arcs_out.clone();
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

impl <A : fmt::Display> fmt::Display for DirectedGraph<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Graph of {} vertices:", self.vertices.len());
        for vertex in self.vertices.iter() {
            for arc in vertex.arcs_out.iter() {
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

pub struct TopologicalIter<'a, A: 'a> {
    graph: &'a DirectedGraph<A>,
    order: Vec<VertexId>
}

impl <'a, A> Iterator for TopologicalIter<'a, A> {
    type Item = &'a Vertex<A>;
    fn next(&mut self) -> Option<&'a Vertex<A>> {
        match self.order.pop() {
            Some(id) => self.graph.vertex(id),
            None => None
        }
    }
}

/// An undirected graph
pub struct UndirectedGraph<A> {
    directed: DirectedGraph<A>
}

impl <A> UndirectedGraph<A> {

    /// Connects two vertices bidirectionally
    pub fn connect_undirected(&mut self, one: VertexId, other: VertexId, weight: i64) -> Result {
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
            graph.breadth_first_iter(two).map(|vertex| &vertex.value).collect::<Vec<&String>>(),
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
            graph.depth_first_iter(two).map(|vertex| &vertex.value).collect::<Vec<&String>>(),
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

        graph.connect(zero, one, 0).unwrap();
        graph.connect(zero, two, 0).unwrap();
        graph.connect(one, two, 0).unwrap();
        graph.connect(two, three, 0).unwrap();

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

        graph.connect(zero, one, 0).unwrap();
        graph.connect(zero, two, 0).unwrap();
        graph.connect(one, two, 0).unwrap();
        graph.connect(two, zero, 1).unwrap();
        graph.connect(two, three, 0).unwrap();
        graph.connect(three, three, 0).unwrap();

        assert_eq!(graph.in_degree(two), Some(2));
        assert_eq!(graph.out_degree(two), Some(2));
        assert_eq!(graph.in_degree(one), Some(1));
        assert_eq!(graph.out_degree(one), Some(1));
        assert_eq!(graph.in_degree(three), Some(2));
        assert_eq!(graph.out_degree(three), Some(1));
    }

    #[test]
    fn topological_order() {

        let mut graph = DirectedGraph::new();

        let zero = graph.add_vertex("zero".to_string());
        let one = graph.add_vertex("one".to_string());
        let two = graph.add_vertex("two".to_string());
        let three = graph.add_vertex("three".to_string());
        let four = graph.add_vertex("four".to_string());
        let five = graph.add_vertex("five".to_string());

        graph.connect(five, two, 0).unwrap();
        graph.connect(five, zero, 0).unwrap();
        graph.connect(four, zero, 0).unwrap();
        graph.connect(four, one, 0).unwrap();
        graph.connect(two, three, 0).unwrap();
        graph.connect(three, one, 0).unwrap();

        assert_eq!(
            graph.topologically_ordered_iter().expect("Turns out acyclic").map(|v| v.value.to_string()).collect::<Vec<String>>(),
            vec!["five", "four", "two", "three", "one", "zero"]
        )
    }

    #[test]
    fn longest_path() {
        let mut graph = DirectedGraph::new();

        let zero = graph.add_vertex("zero".to_string());
        let one = graph.add_vertex("one".to_string());
        let two = graph.add_vertex("two".to_string());
        let three = graph.add_vertex("three".to_string());
        let four = graph.add_vertex("four".to_string());
        let five = graph.add_vertex("five".to_string());

        graph.connect(zero, one, 5).unwrap();
        graph.connect(zero, two, 3).unwrap();
        graph.connect(one, three, 6).unwrap();
        graph.connect(one, two, 2).unwrap();
        graph.connect(two, four, 4).unwrap();
        graph.connect(two, five, 2).unwrap();
        graph.connect(two, three, 7).unwrap();
        graph.connect(three, five, 1).unwrap();
        graph.connect(three, four, -1).unwrap();
        graph.connect(four, five, -2).unwrap();

        assert_eq!(graph.longest_distance_from(one).unwrap().get(&zero), None);
        assert_eq!(graph.longest_distance_from(one).unwrap()[&one], 0);
        assert_eq!(graph.longest_distance_from(one).unwrap()[&two], 2);
        assert_eq!(graph.longest_distance_from(one).unwrap()[&three], 9);
        assert_eq!(graph.longest_distance_from(one).unwrap()[&four], 8);
        assert_eq!(graph.longest_distance_from(one).unwrap()[&five], 10);
    }
}
