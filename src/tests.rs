use super::*;

type TestMesh = Mesh;

#[test]
fn basic_debug_printing() {
    let edge = Edge::default();
    println!("{:?}", edge);
    let vertex = Vertex::default();
    println!("{:?}", vertex);
    let face = Face::default();
    println!("{:?}", face);
    let mesh = TestMesh::new();
    println!("{:?}", mesh);

}

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
        mesh.edge_list.len() == 1 &&
            mesh.edge_list[0].is_valid() == false
    };
    assert! {
        mesh.vertex_list.len() == 1 &&
            mesh.vertex_list[0].is_valid() == false
    };
    assert! {
        mesh.face_list.len() == 1 &&
            mesh.face_list[0].is_valid() == false
    };
}

#[test]
fn can_iterate_over_faces() {
    let mut mesh = TestMesh::new();
    mesh.face_list.push(Face::new(1));
    mesh.face_list.push(Face::new(4));
    mesh.face_list.push(Face::new(7));

    assert!(mesh.face_list.len() == 4);

    let mut faces_iterated_over = 0;

    for index in mesh.faces() {
        let face = mesh.face(index);
        assert!(face.is_valid());
        faces_iterated_over += 1;
    }

    assert!(faces_iterated_over == 3);
}

#[test]
fn can_iterate_over_edges_of_face() {
    let mut mesh = TestMesh::new();
    mesh.vertex_list.push(Vertex::new(1));
    mesh.vertex_list.push(Vertex::new(2));
    mesh.vertex_list.push(Vertex::new(3));
    mesh.edge_list.push(Edge {
        twin_index: INVALID_COMPONENT_INDEX,
        next_index: 2,
        prev_index: 3,
        face_index: 1,
        vertex_index: 1
    });
    mesh.edge_list.push(Edge {
        twin_index: INVALID_COMPONENT_INDEX,
        next_index: 3,
        prev_index: 1,
        face_index: 1,
        vertex_index: 2
    });
    mesh.edge_list.push(Edge {
        twin_index: INVALID_COMPONENT_INDEX,
        next_index: 1,
        prev_index: 2,
        face_index: 1,
        vertex_index: 3
    });
    mesh.face_list.push(Face::new(1));

    assert!(mesh.vertex_list.len() == 4);
    assert!(mesh.edge_list.len() == 4);
    assert!(mesh.face_list.len() == 2);

    let mut faces_iterated_over = 0;
    let mut edges_iterated_over = 0;

    for face_index in mesh.faces() {
        let face = mesh.face(face_index);
        assert!(face.is_valid());
        faces_iterated_over += 1;

        for edge_index in mesh.edges(face) {
            let edge = mesh.edge(edge_index);
            assert!(edge.is_valid());
            edges_iterated_over += 1;
        }
    }

    assert!(faces_iterated_over == 1);
    assert!(edges_iterated_over == 3);
}
