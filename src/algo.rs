use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

pub(crate) fn bfs(graph: HashMap<i32, Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut bfs = vec![];
    let mut q = Vec::new();
    q.push(start);
    visited.insert(start);
    while q.len() > 0 {
        let current_pos = q.remove(0);
        bfs.push(current_pos);
        for i in graph.get(&current_pos).unwrap() {
            if !visited.contains(i) {
                q.push(*i);
                visited.insert(*i);
            }
        }
    }
    bfs
}

pub(crate) fn dfs(graph: HashMap<i32, Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut dfs = vec![];
    let mut stack = Vec::new();
    stack.push(start);
    while stack.len() > 0 {
        let current = stack.pop().unwrap();
        if !visited.contains(&current) {
            dfs.push(current);
            visited.insert(current);
        }
        for i in graph.get(&current).unwrap() {
            if !visited.contains(i) {
                stack.push(*i);
            }
        }
    }
    dfs
}

pub(crate) fn top_sort(graph: HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut top_sort = vec![];
    for (i, j) in &graph {
        if !visited.contains(i) {
            top_sort_util(i, &mut visited, &mut top_sort, &graph)
        }
    }
    top_sort
}

fn top_sort_util(node: &i32, visited: &mut HashSet<i32>, top_sort: &mut Vec<i32>, graph: &HashMap<i32, Vec<i32>>) {
    visited.insert(*node);
    if graph.contains_key(node) {
        for i in graph.get(node).unwrap() {
            if !visited.contains(i) {
                top_sort_util(i, visited, top_sort, graph);
            }
        }
    }
    top_sort.push(*node);
}

pub(crate) fn dijkstra(graph: Vec<Vec<i32>>, src: i32) -> Vec<i32> {
    let V = graph.len();
    let max = i32::MAX;
    let mut dist = vec![max; V];
    dist[src as usize] = 0;
    let mut spt_set = vec![false; V];
    for cout in 0..V {
        let u = min_distance(&dist, &spt_set, V);
        spt_set[u] = true;
        for v in 0..V {
            if graph[u][v] > 0 && spt_set[v] == false && dist[v] > dist[u] + graph[u][v] {
                dist[v] = dist[u] + graph[u][v];
            }
        }
    }
    dist
}

pub(crate) fn dijkstra_path(graph: Vec<Vec<i32>>, src: i32, end: i32) -> Vec<i32> {
    let V = graph.len();
    let max = i32::MAX;
    let mut parent: HashMap<i32, i32> = HashMap::new();
    parent.insert(src, -1);
    let mut dist = vec![max; V];
    dist[src as usize] = 0;
    let mut spt_set = vec![false; V];
    for cout in 0..V {
        let u = min_distance(&dist, &spt_set, V);
        spt_set[u] = true;
        for v in 0..V {
            if graph[u][v] > 0 && spt_set[v] == false && dist[v] > dist[u] + graph[u][v] {
                dist[v] = dist[u] + graph[u][v];
                parent.insert(v as i32, u as i32);
            }
        }
        if u == end as usize {
            break;
        }
    }
    let mut path = Vec::new();
    get_path(&parent, end, &mut path);
    path.into_iter().rev().collect()
}

fn get_path(parent: &HashMap<i32, i32>, end: i32, path: &mut Vec<i32>) {
    let mut n = end;
    while n != -1 {
        path.push(n);
        n = *parent.get(&n).unwrap();
    }
}

fn min_distance(dist: &Vec<i32>, spt_set: &Vec<bool>, V: usize) -> usize {
    let mut min = i32::MAX;
    let mut min_index = 0;
    for v in 0..V {
        if dist[v] < min && spt_set[v] == false {
            min = dist[v];
            min_index = v;
        }
    }
    min_index
}

pub(crate) fn prims(graph: Vec<Vec<i32>>) -> Vec<((i32, i32), i32)> {
    let V = graph.len();
    let max = i32::MAX;
    let mut dist = vec![max; V];
    let mut parent: HashMap<i32, i32> = HashMap::new();
    dist[0] = 0;
    let mut spt_set = vec![false; V];
    for cout in 0..V {
        let u = min_distance(&dist, &spt_set, V);
        spt_set[u] = true;
        for v in 0..V {
            if graph[u][v] > 0 && spt_set[v] == false && dist[v] > graph[u][v] {
                dist[v] = graph[u][v];
                parent.insert(v as i32, u as i32);
            }
        }
    }
    let mut mst: Vec<((i32, i32), i32)> = Vec::new();
    for (i, j) in parent {
        let weight = graph[j as usize][i as usize];
        mst.push(((j, i), weight))
    }
    mst
}

pub(crate) fn scc(graph: HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    let mut stack: Vec<i32> = Vec::new();
    let mut visited: HashSet<i32> = HashSet::new();
    for (i, j) in &graph {
        if !visited.contains(i) {
            get_order(i, &mut visited, &mut stack, &graph)
        }
    }
    let gtrans: HashMap<i32, Vec<i32>> = get_transpose(&graph);
    let mut visited: HashSet<i32> = HashSet::new();
    let mut out = vec![];
    while stack.len() > 0 {
        let i = stack.pop().unwrap();
        if !visited.contains(&i) {
            let mut elements = vec![];
            dfs_ssc(i, &mut visited, &mut elements, &gtrans);
            out.push(elements);
        }
    }
    out
}

fn dfs_ssc(v: i32, visited: &mut HashSet<i32>, elements: &mut Vec<i32>, graph: &HashMap<i32, Vec<i32>>) {
    visited.insert(v);
    elements.push(v);
    if graph.contains_key(&v) {
        for i in graph.get(&v).unwrap() {
            if !visited.contains(i) {
                dfs_ssc(*i, visited, elements, graph);
            }
        }
    }
}

fn get_transpose(graph: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, j) in graph {
        for k in j.iter() {
            add_edge(&mut g, *k, *i)
        }
    }
    g
}

fn add_edge(g: &mut HashMap<i32, Vec<i32>>, u: i32, v: i32) {
    if g.contains_key(&u) {
        g.get_mut(&u).unwrap().push(v);
    } else {
        g.insert(u, vec![v]);
    }
}

fn get_order(v: &i32, visited: &mut HashSet<i32>, stack: &mut Vec<i32>, graph: &HashMap<i32, Vec<i32>>) {
    visited.insert(*v);
    if graph.contains_key(v) {
        for i in graph.get(v).unwrap() {
            if !visited.contains(i) {
                get_order(i, visited, stack, graph);
            }
        }
    }
    stack.push(*v);
}

pub(crate) fn scc_threaded(graphin: HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    let (tx, rx) = mpsc::channel();
    let graph = graphin.clone();
    let v = Arc::new(graphin);
    thread::spawn(move || {
        tx.send(get_transpose_threaded(v)).unwrap();
    });
    // let gtrans: HashMap<i32, Vec<i32>> = get_transpose(&graph);
    let mut stack: Vec<i32> = Vec::new();
    let mut visited: HashSet<i32> = HashSet::new();
    for (i, j) in graph.iter() {
        if !visited.contains(i) {
            get_order(i, &mut visited, &mut stack, &graph)
        }
    }
    let mut visited: HashSet<i32> = HashSet::new();
    let mut out = vec![];
    let gtrans = rx.recv().unwrap();
    while stack.len() > 0 {
        let i = stack.pop().unwrap();
        if !visited.contains(&i) {
            let mut elements = vec![];
            dfs_ssc(i, &mut visited, &mut elements, &gtrans);
            out.push(elements);
        }
    }
    out
}

fn get_transpose_threaded(graph: Arc<HashMap<i32, Vec<i32>>>) -> HashMap<i32, Vec<i32>> {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, j) in graph.iter() {
        for k in j.iter() {
            add_edge(&mut g, *k, *i)
        }
    }
    g
}