
//!
//! This is an implementation of an index based half-edge mesh facility.
//!

use std::fmt;


/// An interface for asserting the validity of components in the mesh.
pub trait Validation {
    /// A general blanket test for validity
    fn is_valid(&self) -> bool;
}


/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_INDEX: usize = 0;

/// Type alias for indices into vertex storage
pub type VertexIndex = usize;

/// Type alias for indices into vertex attribute storage
pub type VertexAttributeIndex = usize;

/// Type alias for indices into edge storage
pub type EdgeIndex = usize;

/// Type alias for indices into face storage
pub type FaceIndex = usize;


/// Represents the point where two edges meet.
#[derive(Default, Debug)]
pub struct Vertex {
    /// Index of the outgoing edge
    pub edge_index: EdgeIndex,
    /// Index of this vertex's attributes
    pub attr_index: VertexAttributeIndex,
}

impl Vertex {
    pub fn new(edge_index: EdgeIndex) -> Vertex {
        Vertex {
            edge_index: edge_index,
            attr_index: INVALID_COMPONENT_INDEX
        }
    }
}

impl Validation for Vertex {
    /// A vertex is considered "valid" as long as it as an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.edge_index != INVALID_COMPONENT_INDEX /*&&
            self.attr_index != INVALID_COMPONENT_INDEX*/
    }
}


/// The principle component in a half-edge mesh.
#[derive(Default, Debug)]
pub struct Edge {
    /// The adjacent or 'twin' half-edge
    pub twin_index: EdgeIndex,
    /// The index of the next edge in the loop
    pub next_index: EdgeIndex,
    /// The index of the previous edge in the loop
    pub prev_index: EdgeIndex,

    /// The index of the face this edge loop defines
    pub face_index: FaceIndex,

    /// The index of the Vertex for this edge.
    pub vertex_index: VertexIndex,
}

impl Validation for Edge {
    /// An edge is generally considered "valid" as long as it has a
    /// vertex and a face index other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.vertex_index != INVALID_COMPONENT_INDEX &&
            self.face_index != INVALID_COMPONENT_INDEX
    }
}


/// A face is defined by the looping connectivity of edges.
#[derive(Default, Debug)]
pub struct Face {
    /// The "root" of an edge loop that defines this face.
    pub edge_index: EdgeIndex,
}

impl Face {
    pub fn new(edge_index: EdgeIndex) -> Face {
        Face {
            edge_index
        }
    }
}

impl Validation for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.edge_index != INVALID_COMPONENT_INDEX
    }
}

/// Function set for operations related to the Face struct
#[derive(Debug)]
pub struct FaceFn<'a> {
    mesh: &'a Mesh,
    pub index: FaceIndex
}

/// Implements the fundamental storage operations and represents the principle
/// grouping of all components.
pub struct Mesh {
    pub edge_list: Vec<Edge>,
    pub vertex_list: Vec<Vertex>,
    pub face_list: Vec<Face>
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Half-Edge Mesh {{ {} vertices, {} edges, {} faces }}",
               self.vertex_list.len(), self.edge_list.len(), self.face_list.len())
    }
}

impl Mesh {
    /// Creates a new Mesh with an initial component added to each Vec.
    ///
    /// The idea behind having a single invalid component at the front of each
    /// Vec comes from the blog http://ourmachinery.com/post/defaulting-to-zero/
    // Not sure though it's something we need to do in Rust given `Option<T>`
    // however, it's nice to have a simple constant value to represent the
    // invalid index.
    pub fn new() -> Mesh {
        Mesh {
            edge_list: vec! [
                Edge::default()
            ],
            vertex_list: vec! [
                Vertex::default()
            ],
            face_list: vec! [
                Face::default()
            ]
        }
    }

    pub fn set_twin_edges(&mut self, e1: EdgeIndex, e2: EdgeIndex) {
        // TODO: we should verify vertex correlation
        if let Some(ref mut edge1) = self.edge_mut(e1) {
            edge1.twin_index = e2;
        }
        if let Some(ref mut edge2) = self.edge_mut(e2) {
            edge2.twin_index = e1;
        }
    }

    pub fn connect_edges(&mut self, e1: EdgeIndex, e2: EdgeIndex) {
        if let Some(ref mut edge1) = self.edge_mut(e1) {
            edge1.next_index = e2;
        }
        if let Some(ref mut edge2) = self.edge_mut(e2) {
            edge2.prev_index = e1;
        }
    }

    pub fn edge_from_vertex(&mut self, vert: VertexIndex) -> EdgeIndex {
        self.add_edge(Edge {
            twin_index: INVALID_COMPONENT_INDEX,
            next_index: INVALID_COMPONENT_INDEX,
            prev_index: INVALID_COMPONENT_INDEX,
            face_index: INVALID_COMPONENT_INDEX,
            vertex_index: vert
        })
    }

    pub fn edge_from_twin(&mut self, twin: EdgeIndex) -> EdgeIndex {
        let result = self.add_edge(Edge {
            twin_index: twin,
            next_index: INVALID_COMPONENT_INDEX,
            prev_index: INVALID_COMPONENT_INDEX,
            face_index: INVALID_COMPONENT_INDEX,
            vertex_index: INVALID_COMPONENT_INDEX
        });

        //

        return result;
    }

    pub fn close_edge_loop(&mut self, vert: VertexIndex, next: EdgeIndex, prev: EdgeIndex) -> EdgeIndex {
        let result = self.add_edge(Edge {
            twin_index: INVALID_COMPONENT_INDEX,
            next_index: next,
            prev_index: prev,
            face_index: INVALID_COMPONENT_INDEX,
            vertex_index: vert
        });

        return result;
    }

    pub fn add_edge(&mut self, edge: Edge) -> EdgeIndex {
        let result: EdgeIndex = self.edge_list.len();

        self.edge_list.push(edge);

        return result;
    }

    pub fn add_triangle(&mut self, a: VertexIndex, b: VertexIndex, c: VertexIndex) -> FaceIndex {
        let result: FaceIndex = self.face_list.len();

        //let e1 = self.add_edge(Edge {});
        return result;
    }

    pub fn add_adjacent_triangle(&mut self, a: VertexIndex, twin_edge: EdgeIndex) -> FaceIndex {
        let result: FaceIndex = self.face_list.len();

        return result;
    }

    pub fn faces(&self) -> FaceIterator {
        FaceIterator::new(self.face_list.len())
    }

    pub fn edges(&self, face: &Face) -> FaceEdgeIterator {
        FaceEdgeIterator::new(face.edge_index, &self.edge_list)
    }

    pub fn vertices(&self, face: &Face) -> FaceVertexIterator {
        FaceVertexIterator::new(face.edge_index, &self.edge_list)
    }

    pub fn face(&self, index: FaceIndex) -> &Face {
        if let Some(result) = self.face_list.get(index) {
            result
        } else {
            &self.face_list[0]
        }
    }

    pub fn edge(&self, index: EdgeIndex) -> &Edge {
        if let Some(result) = self.edge_list.get(index) {
            result
        } else {
            &self.edge_list[0]
        }
    }

    pub fn vertex(&self, index: VertexIndex) -> &Vertex {
        if let Some(result) = self.vertex_list.get(index) {
            result
        } else {
            &self.vertex_list[0]
        }
    }

    pub fn face_mut(&mut self, index: FaceIndex) -> Option<&mut Face> {
        if index == INVALID_COMPONENT_INDEX {
            None
        } else {
            self.face_list.get_mut(index)
        }
    }

    pub fn edge_mut(&mut self, index: EdgeIndex) -> Option<&mut Edge> {
        if index == INVALID_COMPONENT_INDEX {
            None
        } else {
            self.edge_list.get_mut(index)
        }
    }

    pub fn vertex_mut(&mut self, index: VertexIndex) -> Option<&mut Vertex> {
        if index == INVALID_COMPONENT_INDEX {
            None
        } else {
            self.vertex_list.get_mut(index)
        }
    }
}

// yeah yeah yeah, I know this is copypasta...
pub struct FaceVertexIterator<'mesh> {
    edge_list: &'mesh Vec<Edge>,
    initial_index: EdgeIndex,
    current_index: EdgeIndex
}

impl<'mesh> FaceVertexIterator<'mesh> {
    pub fn new(index: EdgeIndex, edge_list: &'mesh Vec<Edge>) -> FaceVertexIterator {
        FaceVertexIterator {
            edge_list: edge_list,
            initial_index: index,
            current_index: INVALID_COMPONENT_INDEX
        }
    }
}

impl<'mesh> Iterator for FaceVertexIterator<'mesh> {
    type Item = VertexIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == INVALID_COMPONENT_INDEX {
            self.current_index = self.initial_index;
            Some(self.current_index)
        } else {
            self.edge_list.get(self.current_index)
                .and_then(|last_edge| {
                    self.current_index = last_edge.next_index;
                    if self.current_index == self.initial_index {
                        None
                    } else {
                        self.edge_list.get(self.current_index)
                            .map(|current_edge| current_edge.vertex_index)
                    }
                })
        }
    }
}

pub struct FaceEdgeIterator<'mesh> {
    edge_list: &'mesh Vec<Edge>,
    initial_index: EdgeIndex,
    current_index: EdgeIndex
}

impl<'mesh> FaceEdgeIterator<'mesh> {
    pub fn new(index: EdgeIndex, edge_list: &'mesh Vec<Edge>) -> FaceEdgeIterator {
        FaceEdgeIterator {
            edge_list: edge_list,
            initial_index: index,
            current_index: INVALID_COMPONENT_INDEX
        }
    }
}

impl<'mesh> Iterator for FaceEdgeIterator<'mesh> {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == INVALID_COMPONENT_INDEX {
            self.current_index = self.initial_index;
            Some(self.current_index)
        } else {
            self.edge_list.get(self.current_index).and_then(|current_edge| {
                self.current_index = current_edge.next_index;
                if self.current_index == self.initial_index {
                    None
                } else {
                    Some(self.current_index)
                }
            })
        }
    }
}

pub struct FaceIterator {
    face_count: usize,
    previous_index: FaceIndex
}

impl FaceIterator {
    pub fn new(face_count: usize) -> FaceIterator {
        FaceIterator {
            face_count: face_count,
            previous_index: 0
        }
    }
}

impl Iterator for FaceIterator {
    type Item = FaceIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous_index += 1;
        if self.previous_index >= self.face_count {
            None
        } else {
            Some(self.previous_index)
        }
    }
}


#[cfg(test)]
mod tests;
