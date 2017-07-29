# Hedge Changelog

## 0.0.9

`EdgeIndex`, `FaceIndex`, and `VertexIndex` are now structs instead of type aliases.

## 0.0.8

- Added method `Edge::is_boundary`
- Added method `Edge::is_connected`
- Added method `Mesh::remove_face`
- Added method `Mesh::remove_edge`
- Added *unimplemented* method `Mesh::remove_vertex`
- Added cgmath dependency
- Moved repo to github

## 0.0.7

- Introducing Changelog.md
- Fixed some typos in documentation
- Updated documentation when missing notices about debug assertions
- Added `Validation` implementations for the function set structs
- Added method `Mesh::assign_face_to_loop`
- Added method `Mesh::add_polygon`

## 0.0.6 - 0.0.1

- Core api exploration, iterators, function set api, and basic primitive operations
