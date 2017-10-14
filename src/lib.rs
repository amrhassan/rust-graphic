
use std::fmt;
use std::result;

pub type Result = result::Result<(), String>;

#[derive(Debug, Copy, Clone)]
pub struct VertexId { value: usize }

struct Incidence {
    other: VertexId,
    weight: f64
}

struct Vertex<A> {
    value: A,
    adjacents: Vec<Incidence>
}

struct DirectedGraph<A> {
    vertices: Vec<Vertex<A>>
}

impl <A> DirectedGraph<A> {

    pub fn new() -> DirectedGraph<A> {
        DirectedGraph { vertices: Vec::new() }
    }

    /// Retrieves a vertex value
    pub fn vertex_value(&self, id: VertexId) -> Option<&A> {
        self.vertices.get(id.value).map(|vertex| &vertex.value)
    }

    /// Retrieves a vertex value
    pub fn vertex_value_mut(&mut self, id: VertexId) -> Option<&mut A> {
        self.vertices.get_mut(id.value).map(|vertex| &mut vertex.value)
    }

    /// Retrieves the vertex value from the graph
    pub fn add_vertex(&mut self, value: A) -> VertexId {
        self.vertices.push(Vertex { value: value, adjacents: Vec::new() });
        VertexId { value: self.vertices.len()-1 }
    }

    /// Connects two vertices
    pub fn connect(&mut self, from: VertexId, to: VertexId, weight: f64) -> Result {
        match self.vertices.get_mut(from.value) {
            Some(vertex) => {
                let incident = Incidence { other: to, weight: weight };
                vertex.adjacents.push(incident);
                Ok(())
            },
            None => Err(format!("{:?} does not exist", from))
        }
    }
}

impl <A : fmt::Display> fmt::Display for DirectedGraph<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Graph of {} vertices:", self.vertices.len());
        for (vertex, vertex_id) in self.vertices.iter().zip(0..) {
            for incidence in vertex.adjacents.iter() {
                let from_vertex_id = VertexId { value: vertex_id };
                let _ = writeln!(f, "\t ({}:{}) -(weight: {})-> ({}:{})",
                                 from_vertex_id,
                                 self.vertex_value(from_vertex_id).expect("Failed to get vertex value"),
                                 incidence.weight,
                                 incidence.other,
                                 self.vertex_value(incidence.other).expect("Failed to get vertex value"));
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

struct UndirectedGraph<A> {
    directed: DirectedGraph<A>
}

impl <A> UndirectedGraph<A> {

    /// Connects two vertices bidirectionally
    pub fn connect_undirected(&mut self, one: VertexId, other: VertexId, weight: f64) -> Result {
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
    fn it_works() {

        let mut graph = UndirectedGraph::new();

        let zero = graph.add_vertex("zero");
        let one = graph.add_vertex("one");
        let two = graph.add_vertex("two");
        let three = graph.add_vertex("three");
        let four = graph.add_vertex("four");

        graph.connect_undirected(zero, one, 0.1);
        graph.connect_undirected(zero, four, 0.2);
        graph.connect_undirected(one, two, 0.3);
        graph.connect_undirected(one, three, 0.5);
        graph.connect_undirected(one, four, 0.6);
        graph.connect_undirected(two, three, 0.7);
        graph.connect_undirected(two, four, 0.8);

        println!("{}", graph);
        assert!(false)
    }
}
