
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

impl Validation for Vertex {
    /// A vertex is considered "valid" as long as it as an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.edge_index != INVALID_COMPONENT_INDEX &&
            self.attr_index != INVALID_COMPONENT_INDEX
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
pub struct FaceFn<'a>
{
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

    pub fn face(&self, index: FaceIndex) -> &Face {
        if let Some(result) = self.face_list.get(index) {
            result
        } else {
            &self.face_list[0]
        }
    }

    pub fn face_mut(&mut self, index: FaceIndex) -> Option<&mut Face> {
        if index == INVALID_COMPONENT_INDEX {
            None
        } else {
            self.face_list.get_mut(index)
        }
    }
}

impl<'a> IntoIterator for &'a Mesh {
    type Item = &'a Face;
    type IntoIter = FaceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FaceIterator::new(&self)
    }
}

pub struct FaceIterator<'a> {
    mesh: &'a Mesh,
    previous_index: FaceIndex
}

impl<'a> FaceIterator<'a> {
    pub fn new(mesh: &'a Mesh) -> FaceIterator {
        FaceIterator {
            mesh: mesh,
            previous_index: 0
        }
    }
}

impl<'a> Iterator for FaceIterator<'a> {
    type Item = &'a Face;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous_index += 1;
        self.mesh.face_list.get(self.previous_index)
    }
}


#[cfg(test)]
mod tests;
