use std::collections::HashMap;

use pyo3::prelude::*;

mod algo;

/// Breadth first search
#[pyfunction]
fn bfs(graph: HashMap<i32, Vec<i32>>, start: i32) -> PyResult<Vec<i32>> {
    Ok(algo::bfs(graph, start))
}

/// Depth first search
#[pyfunction]
fn dfs(graph: HashMap<i32, Vec<i32>>, start: i32) -> PyResult<Vec<i32>> {
    Ok(algo::dfs(graph, start))
}

/// topological sort
#[pyfunction]
fn top_sort(graph: HashMap<i32, Vec<i32>>) -> PyResult<Vec<i32>> {
    Ok(algo::top_sort(graph))
}

/// Djikstra shortes path to every node
#[pyfunction]
fn dijkstra(graph: Vec<Vec<i32>>, start_node: i32) -> PyResult<Vec<i32>> {
    Ok(algo::dijkstra(graph, start_node))
}

/// Djikstra shortes path from start node to end node
#[pyfunction]
fn dijkstra_path(graph: Vec<Vec<i32>>, start_node: i32, end_node: i32) -> PyResult<Vec<i32>> {
    Ok(algo::dijkstra_path(graph, start_node, end_node))
}

/// Minimum spanning tree Kruskal
#[pyfunction]
fn prims(graph: Vec<Vec<i32>>) -> PyResult<Vec<((i32, i32), i32)>> {
    Ok(algo::prims(graph))
}

/// Strongly connected component
#[pyfunction]
fn scc(graph: HashMap<i32, Vec<i32>>) -> PyResult<Vec<Vec<i32>>> {
    Ok(algo::scc(graph))
}

/// Strongly connected component threaded
#[pyfunction]
fn scc_threaded(graph: HashMap<i32, Vec<i32>>) -> PyResult<Vec<Vec<i32>>> {
    Ok(algo::scc_threaded(graph))
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rustyGraph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bfs, m)?)?;
    m.add_function(wrap_pyfunction!(dfs, m)?)?;
    m.add_function(wrap_pyfunction!(top_sort, m)?)?;
    m.add_function(wrap_pyfunction!(dijkstra, m)?)?;
    m.add_function(wrap_pyfunction!(dijkstra_path, m)?)?;
    m.add_function(wrap_pyfunction!(prims, m)?)?;
    m.add_function(wrap_pyfunction!(scc, m)?)?;
    m.add_function(wrap_pyfunction!(scc_threaded, m)?)?;

    Ok(())
}