
//!
//! An index based half-edge mesh implementation.
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
pub struct FaceFn<'mesh> {
    mesh: &'mesh Mesh,
    face: &'mesh Face,
    pub index: FaceIndex
}

/// Function set for operations related to the Vertex struct
#[derive(Debug)]
pub struct VertexFn<'mesh> {
    mesh: &'mesh Mesh,
    vertex: &'mesh Vertex,
    pub index: VertexIndex
}

/// Function set for operations related to the Edge struct
#[derive(Debug)]
pub struct EdgeFn<'mesh> {
    mesh: &'mesh Mesh,
    edge: &'mesh Edge,
    pub index: EdgeIndex
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

    /// Mark the two edges as adjacent twins.
    ///
    /// In order for this to be valid each edge should be connected in such a way
    /// that the vertex of each is the same as the vertex of the next edge of each.
    ///
    /// So: `A->Next->Vertex == B->Vertex` && `B->Next->Vertex == A->Vertex`
    ///
    /// _In debug builds we assert the provided indices are valid._
    pub fn set_twin_edges(&mut self, e1: EdgeIndex, e2: EdgeIndex) {
        debug_assert!(e1 != INVALID_COMPONENT_INDEX && e2 != INVALID_COMPONENT_INDEX);
        // TODO: Disabling this for the moment because it would prevent the use
        //       of the `edge_from_twin` method.
        // debug_assert! {
        //     self.edge(e1).vertex_index == self.edge( self.edge(e2).next_index ).vertex_index
        // };
        // debug_assert! {
        //     self.edge(e2).vertex_index == self.edge( self.edge(e1).next_index ).vertex_index
        // };
        if let Some(ref mut edge1) = self.edge_mut(e1) {
            edge1.twin_index = e2;
        }
        if let Some(ref mut edge2) = self.edge_mut(e2) {
            edge2.twin_index = e1;
        }
    }

    /// Connects the two edges as part of an edge loop.
    ///
    /// _In debug builds we assert that neither index is not the default index._
    pub fn connect_edges(&mut self, prev: EdgeIndex, next: EdgeIndex) {
        debug_assert!(prev != INVALID_COMPONENT_INDEX && next != INVALID_COMPONENT_INDEX);
        if let Some(ref mut prev_edge) = self.edge_mut(prev) {
            prev_edge.next_index = next;
        }
        if let Some(ref mut next_edge) = self.edge_mut(next) {
            next_edge.prev_index = prev;
        }
    }

    /// Create a new edge from the specified vertex.
    ///
    /// _In debug builds we assert that the vertex index is not the default index._
    pub fn edge_from_vertex(&mut self, vert: VertexIndex) -> EdgeIndex {
        debug_assert!(vert != INVALID_COMPONENT_INDEX);
        self.add_edge(Edge {
            twin_index: INVALID_COMPONENT_INDEX,
            next_index: INVALID_COMPONENT_INDEX,
            prev_index: INVALID_COMPONENT_INDEX,
            face_index: INVALID_COMPONENT_INDEX,
            vertex_index: vert
        })
    }

    /// Create a new edge as a twin of the specified edge
    ///
    /// _In debug builds we assert that the twin index is not the default index
    /// and that the twins next index is not the default index (since we need
    /// that edge to find the correct vertex index)._
    pub fn edge_from_twin(&mut self, twin: EdgeIndex) -> EdgeIndex {
        debug_assert!(twin != INVALID_COMPONENT_INDEX);
        debug_assert!(self.edge(twin).next_index != INVALID_COMPONENT_INDEX);
        let vert = self.edge( self.edge(twin).next_index ).vertex_index;
        let result = self.edge_from_vertex(vert);
        self.set_twin_edges(result, twin);
        return result;
    }

    /// Create a new edge connected to the previous edge specified.
    ///
    /// _In debug builds we assert that the indices specified are valid._
    pub fn extend_edge_loop(&mut self, vert: VertexIndex, prev: EdgeIndex) -> EdgeIndex {
        debug_assert!(prev != INVALID_COMPONENT_INDEX && vert != INVALID_COMPONENT_INDEX);
        let result = match vert {
            INVALID_COMPONENT_INDEX => {
                debug_assert!(self.edge(prev).twin_index != INVALID_COMPONENT_INDEX);
                let vert = self.edge( self.edge(prev).twin_index ).vertex_index;
                self.edge_from_vertex(vert)
            },
            _ => self.edge_from_vertex(vert)
        };
        self.connect_edges(prev, result);
        return result;
    }

    /// Create a new edge, closing an edge loop, using the `prev` and `next` indices provided.
    ///
    /// _In debug builds we assert that all specified indices are valid._
    pub fn close_edge_loop(&mut self, vert: VertexIndex, prev: EdgeIndex, next: EdgeIndex) -> EdgeIndex {
        debug_assert! {
            vert != INVALID_COMPONENT_INDEX &&
                prev != INVALID_COMPONENT_INDEX &&
                next != INVALID_COMPONENT_INDEX
        };
        let result = self.edge_from_vertex(vert);
        self.connect_edges(prev, result);
        self.connect_edges(result, next);
        return result;
    }

    /// Adds the provided `Edge` to the mesh and returns it's `EdgeIndex`
    pub fn add_edge(&mut self, edge: Edge) -> EdgeIndex {
        let result: EdgeIndex = self.edge_list.len();
        self.edge_list.push(edge);
        return result;
    }

    /// Creates a new face and associated edges with the given vertex indices.
    /// Returns the index of the newly added face.
    ///
    /// _In debug builds we assert that all provided indices are valid._
    pub fn add_triangle(&mut self, a: VertexIndex, b: VertexIndex, c: VertexIndex) -> FaceIndex {
        debug_assert! {
            a != INVALID_COMPONENT_INDEX &&
                b != INVALID_COMPONENT_INDEX &&
                c != INVALID_COMPONENT_INDEX
        };
        let result: FaceIndex = self.face_list.len();

        let e1 = self.edge_from_vertex(a);
        let e2 = self.extend_edge_loop(b, e1);
        let e3 = self.close_edge_loop(c, e2, e1);

        self.edge_mut(e1).map(|e| e.face_index = result);
        self.edge_mut(e2).map(|e| e.face_index = result);
        self.edge_mut(e3).map(|e| e.face_index = result);

        self.face_list.push(Face::new(e1));

        return result;
    }

    /// Creates a new face and associated edges with the given a vertex index and a twin edge index.
    /// Returns the index of the newly added face.
    ///
    /// _In debug builds we assert that the all provided indices are valid._
    pub fn add_adjacent_triangle(&mut self, a: VertexIndex, twin_edge: EdgeIndex) -> FaceIndex {
        debug_assert! {
            a != INVALID_COMPONENT_INDEX && twin_edge != INVALID_COMPONENT_INDEX
        };
        let result: FaceIndex = self.face_list.len();

        let e1 = self.edge_from_twin(twin_edge);
        let e2 = self.extend_edge_loop(INVALID_COMPONENT_INDEX, e1);
        let e3 = self.close_edge_loop(a, e2, e1);

        self.edge_mut(e1).map(|e| e.face_index = result);
        self.edge_mut(e2).map(|e| e.face_index = result);
        self.edge_mut(e3).map(|e| e.face_index = result);

        self.face_list.push(Face::new(e1));

        return result;
    }

    /// Returns a `FaceIterator` for this mesh.
    ///
    /// ```
    /// for index in mesh.faces() {
    ///    let face = mesh.face(index);
    /// }
    /// ```
    pub fn faces(&self) -> FaceIterator {
        FaceIterator::new(self.face_list.len())
    }

    /// Returns an `EdgeLoopIterator` for the edges around the specified face.
    ///
    /// ```
    /// for findex in mesh.faces() {
    ///    let face = mesh.face(findex);
    ///    for eindex in mesh.edges(face) {
    ///        let edge = mesh.edge(eindex);
    ///    }
    /// }
    /// ```
    pub fn edges(&self, face: &Face) -> EdgeLoopIterator {
        EdgeLoopIterator::new(face.edge_index, &self.edge_list)
    }

    /// Returns an `EdgeLoopVertexIterator` for the vertices around the specified face.
    ///
    /// ```
    /// for findex in mesh.faces() {
    ///    let face = mesh.face(findex);
    ///    for vindex in mesh.vertices(face) {
    ///        let vertex = mesh.vertex(vindex);
    ///    }
    /// }
    /// ```
    pub fn vertices(&self, face: &Face) -> EdgeLoopVertexIterator {
        EdgeLoopVertexIterator::new(face.edge_index, &self.edge_list)
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

/// An iterator that walks an edge loop around a face returning each `VertexIndex` in the loop.
// yeah yeah yeah, I know this is copypasta...
pub struct EdgeLoopVertexIterator<'mesh> {
    edge_list: &'mesh Vec<Edge>,
    initial_index: EdgeIndex,
    current_index: EdgeIndex
}

impl<'mesh> EdgeLoopVertexIterator<'mesh> {
    pub fn new(index: EdgeIndex, edge_list: &'mesh Vec<Edge>) -> EdgeLoopVertexIterator {
        EdgeLoopVertexIterator {
            edge_list: edge_list,
            initial_index: index,
            current_index: INVALID_COMPONENT_INDEX
        }
    }
}

impl<'mesh> Iterator for EdgeLoopVertexIterator<'mesh> {
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

/// An iterator that walks an edge loop around a face returning each `EdgeIndex` in the loop.
pub struct EdgeLoopIterator<'mesh> {
    edge_list: &'mesh Vec<Edge>,
    initial_index: EdgeIndex,
    current_index: EdgeIndex
}

impl<'mesh> EdgeLoopIterator<'mesh> {
    pub fn new(index: EdgeIndex, edge_list: &'mesh Vec<Edge>) -> EdgeLoopIterator {
        EdgeLoopIterator {
            edge_list: edge_list,
            initial_index: index,
            current_index: INVALID_COMPONENT_INDEX
        }
    }
}

impl<'mesh> Iterator for EdgeLoopIterator<'mesh> {
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

/// An iterator that returns the `FaceIndex` of every Face in the mesh.
///
/// Currently this does not iterate using connectivity information but will
/// perhaps do this in the future.
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

// TODO: iterate over faces based on connectivity?
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
