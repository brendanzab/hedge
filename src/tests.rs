use super::*;

#[test]
fn default_edge_is_invalid() {
    let edge = Edge::default();
    assert!(edge.is_valid() == false);
}

#[test]
fn default_vertex_is_invalid() {
    let vertex = Vertex::default();
    assert!(vertex.is_valid() == false);
}

#[test]
fn default_face_is_invalid() {
    let face = Face::default();
    assert!(face.is_valid() == false);
}

#[test]
fn initial_mesh_has_default_elements() {
    let mesh = Mesh::new();
    assert! {
        mesh.edges.len() == 1 &&
            mesh.edges[0].is_valid() == false
    };
    assert! {
        mesh.vertices.len() == 1 &&
            mesh.vertices[0].is_valid() == false
    };
    assert! {
        mesh.faces.len() == 1 &&
            mesh.faces[0].is_valid() == false
    };
}

#[test]
fn can_iterate_over_faces() {
    let mut mesh = Mesh::new();
    mesh.faces.push(Face::new(1));
    mesh.faces.push(Face::new(4));
    mesh.faces.push(Face::new(7));

    assert!(mesh.faces.len() == 4);

    let mut faces_iterated_over = 0;

    for face in mesh.into_iter() {
        assert!(face.is_valid());
        faces_iterated_over += 1;
    }

    assert!(faces_iterated_over == 3);
}
