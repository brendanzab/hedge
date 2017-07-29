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
fn index_types_are_invalid_by_default() {
    let vert = EdgeIndex::default();
    let edge = EdgeIndex::default();
    assert!(!vert.is_valid());
    assert!(!edge.is_valid());
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
    mesh.face_list.push(Face::new(EdgeIndex(1)));
    mesh.face_list.push(Face::new(EdgeIndex(4)));
    mesh.face_list.push(Face::new(EdgeIndex(7)));

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
    mesh.vertex_list.push(Vertex::new(EdgeIndex(1)));
    mesh.vertex_list.push(Vertex::new(EdgeIndex(2)));
    mesh.vertex_list.push(Vertex::new(EdgeIndex(3)));
    mesh.edge_list.push(Edge {
        twin_index: EdgeIndex::default(),
        next_index: EdgeIndex(2),
        prev_index: EdgeIndex(3),
        face_index: FaceIndex(1),
        vertex_index: VertexIndex(1)
    });
    mesh.edge_list.push(Edge {
        twin_index: EdgeIndex::default(),
        next_index: EdgeIndex(3),
        prev_index: EdgeIndex(1),
        face_index: FaceIndex(1),
        vertex_index: VertexIndex(2)
    });
    mesh.edge_list.push(Edge {
        twin_index: EdgeIndex::default(),
        next_index: EdgeIndex(1),
        prev_index: EdgeIndex(2),
        face_index: FaceIndex(1),
        vertex_index: VertexIndex(3)
    });
    mesh.face_list.push(Face::new(EdgeIndex(1)));

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
    mesh.vertex_list.push(Vertex::new(EdgeIndex(1)));
    mesh.vertex_list.push(Vertex::new(EdgeIndex(2)));
    mesh.vertex_list.push(Vertex::new(EdgeIndex(3)));
    mesh.edge_list.push(Edge {
        twin_index: EdgeIndex::default(),
        next_index: EdgeIndex(2),
        prev_index: EdgeIndex(3),
        face_index: FaceIndex(1),
        vertex_index: VertexIndex(1)
    });
    mesh.edge_list.push(Edge {
        twin_index: EdgeIndex::default(),
        next_index: EdgeIndex(3),
        prev_index: EdgeIndex(1),
        face_index: FaceIndex(1),
        vertex_index: VertexIndex(2)
    });
    mesh.edge_list.push(Edge {
        twin_index: EdgeIndex::default(),
        next_index: EdgeIndex(1),
        prev_index: EdgeIndex(2),
        face_index: FaceIndex(1),
        vertex_index: VertexIndex(3)
    });
    mesh.face_list.push(Face::new(EdgeIndex(1)));

    assert!(mesh.vertex_list.len() == 4);
    assert!(mesh.edge_list.len() == 4);
    assert!(mesh.face_list.len() == 2);

    let mut faces_iterated_over = 0;
    let mut vertices_iterated_over = 0;

    for face_index in mesh.faces() {
        assert!(face_index.is_valid());
        let face = mesh.face(face_index);
        assert!(face.is_valid());
        faces_iterated_over += 1;

        for vertex_index in mesh.vertices(face) {
            assert!(vertex_index.is_valid());
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
        assert!(edge.next_index.is_valid());
        assert!(edge.prev_index.is_valid());
    }

    let twin_a = mesh.face_fn(f1).edge().next().index;
    assert!(twin_a.is_valid());

    let f2 = mesh.add_adjacent_triangle(v3, twin_a);
    for eindex in mesh.edges(mesh.face(f1)) {
        let ref edge = mesh.edge(eindex);
        assert!(edge.next_index.is_valid());
        assert!(edge.prev_index.is_valid());
    }

    let twin_b = mesh.face(f2).edge_index;
    assert!(twin_b.is_valid());

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
        assert!(vertex.edge_index.0 == 1);
        vertex.edge_index = EdgeIndex::default();
    }

    assert!(mesh.face_fn(f1).edge().vertex().edge().index.is_valid() == false);
}

#[test]
fn can_build_a_simple_mesh() {
    let mut mesh = TestMesh::new();

    let v1 = mesh.add_vertex(Vertex::default());
    let v2 = mesh.add_vertex(Vertex::default());
    let v3 = mesh.add_vertex(Vertex::default());
    let v4 = mesh.add_vertex(Vertex::default());

    let f1 = mesh.add_triangle(v1, v2, v3);
    let f2 = {
        let edge_index = mesh.face_fn(f1).edge().index;
        mesh.add_adjacent_triangle(v4, edge_index)
    };
    let f3 = {
        let edge_index = mesh.face_fn(f1).edge().next().index;
        mesh.add_adjacent_triangle(v4, edge_index)
    };
    let f4 = {
        let edge_index = mesh.face_fn(f1).edge().prev().index;
        mesh.add_adjacent_triangle(v4, edge_index)
    };

    // stitch f2-f3
    {
        let edge_a = mesh.face_fn(f2).edge().next().index;
        let edge_b = mesh.face_fn(f3).edge().prev().index;
        mesh.set_twin_edges(edge_a, edge_b);
    }

    // stitch f3-f4
    {
        let edge_a = mesh.face_fn(f3).edge().next().index;
        let edge_b = mesh.face_fn(f4).edge().prev().index;
        mesh.set_twin_edges(edge_a, edge_b);
    }

    // stitch f4-f2
    {
        let edge_a = mesh.face_fn(f4).edge().next().index;
        let edge_b = mesh.face_fn(f2).edge().prev().index;
        mesh.set_twin_edges(edge_a, edge_b);
    }

    assert!(mesh.face_fn(f1).edge().twin().face().index        == f2);
    assert!(mesh.face_fn(f1).edge().next().twin().face().index == f3);
    assert!(mesh.face_fn(f1).edge().prev().twin().face().index == f4);

    assert!(mesh.face_fn(f2).edge().next().twin().face().index == f3);
    assert!(mesh.face_fn(f2).edge().prev().twin().face().index == f4);

    assert!(mesh.face_fn(f3).edge().next().twin().face().index == f4);
    assert!(mesh.face_fn(f3).edge().prev().twin().face().index == f2);

    assert!(mesh.face_fn(f4).edge().next().twin().face().index == f2);
    assert!(mesh.face_fn(f4).edge().prev().twin().face().index == f3);

    assert!(mesh.face_fn(f1).edge().prev().vertex().index == mesh.face_fn(f3).edge().vertex().index);
    assert!(mesh.face_fn(f1).edge().vertex().index        == mesh.face_fn(f4).edge().vertex().index);
    assert!(mesh.face_fn(f1).edge().next().vertex().index == mesh.face_fn(f2).edge().vertex().index);

    assert!(mesh.face_fn(f2).edge().vertex().index == mesh.face_fn(f3).edge().next().vertex().index);
    assert!(mesh.face_fn(f3).edge().vertex().index == mesh.face_fn(f4).edge().next().vertex().index);

    let f2_prev_vert = mesh.face_fn(f2).edge().prev().vertex().index;
    let f3_prev_vert = mesh.face_fn(f3).edge().prev().vertex().index;
    let f4_prev_vert = mesh.face_fn(f4).edge().prev().vertex().index;
    assert! {
        (f2_prev_vert == v4) && (f3_prev_vert == v4) && (f4_prev_vert == v4)
    };
}
