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

#[test]
fn can_iterate_over_vertices_of_face() {
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
    let mut vertices_iterated_over = 0;

    for face_index in mesh.faces() {
        assert!(face_index != INVALID_COMPONENT_INDEX);
        let face = mesh.face(face_index);
        assert!(face.is_valid());
        faces_iterated_over += 1;

        for vertex_index in mesh.vertices(face) {
            assert!(vertex_index != INVALID_COMPONENT_INDEX);
            let vertex = mesh.vertex(vertex_index);
            assert!(vertex.is_valid());
            vertices_iterated_over += 1;
        }
    }

    assert!(faces_iterated_over == 1);
    assert!(vertices_iterated_over == 3);
}

#[test]
fn can_add_triangles_to_mesh() {
    let mut mesh = TestMesh::new();

    let v1 = mesh.add_vertex(Vertex::default());
    let v2 = mesh.add_vertex(Vertex::default());
    let v3 = mesh.add_vertex(Vertex::default());
    let v4 = mesh.add_vertex(Vertex::default());

    let f1 = mesh.add_triangle(v1, v2, v4);
    for eindex in mesh.edges(mesh.face(f1)) {
        let ref edge = mesh.edge(eindex);
        assert!(edge.next_index != INVALID_COMPONENT_INDEX);
        assert!(edge.prev_index != INVALID_COMPONENT_INDEX);
    }

    let twin_a = mesh.face_fn(f1).edge().next().index;
    assert!(twin_a != INVALID_COMPONENT_INDEX);

    let f2 = mesh.add_adjacent_triangle(v3, twin_a);
    for eindex in mesh.edges(mesh.face(f1)) {
        let ref edge = mesh.edge(eindex);
        assert!(edge.next_index != INVALID_COMPONENT_INDEX);
        assert!(edge.prev_index != INVALID_COMPONENT_INDEX);
    }

    let twin_b = mesh.face(f2).edge_index;
    assert!(twin_b != INVALID_COMPONENT_INDEX);

    assert!(mesh.edge(twin_a).twin_index == twin_b);
    assert!(mesh.edge(twin_b).twin_index == twin_a);

    assert!(mesh.edge(twin_a).vertex_index == mesh.edge_fn(twin_b).next().vertex().index);
    assert!(mesh.edge(twin_b).vertex_index == mesh.edge_fn(twin_a).next().vertex().index);
}

#[test]
fn can_walk_and_get_mutable_ref() {
    let mut mesh = TestMesh::new();

    let v1 = mesh.add_vertex(Vertex::default());
    let v2 = mesh.add_vertex(Vertex::default());
    let v3 = mesh.add_vertex(Vertex::default());

    let f1 = mesh.add_triangle(v1, v2, v3);

    {
        let vertex = {
            let index = mesh.face_fn(f1).edge().vertex().index;
            mesh.vertex_mut(index).unwrap()
        };
        assert!(vertex.edge_index == 1);
        vertex.edge_index = INVALID_COMPONENT_INDEX;
    }

    assert!(mesh.face_fn(f1).edge().vertex().edge().index == INVALID_COMPONENT_INDEX);
}
