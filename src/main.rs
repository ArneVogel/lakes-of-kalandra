use image::{GenericImage, RgbImage};
use petgraph::{algo, prelude::*};
use std::collections::HashSet;
use std::env;

// all paths beteen a and b also contain path which break the manhatten distance therefore we need
// to make sure that any node in the path at least touches one other node also in the path
fn verify(ways: &Vec<NodeIndex>, rows: i32, _cols: i32) -> bool {
    let mut seen = HashSet::new();

    for n in ways {
        let mut count = 0;
        let n = n.index() as i32;
        let mut neighbors = Vec::new(); // [n - 1, n + 1, n - cols, n + cols];
                                        //
        if n % rows != 0 {
            neighbors.push(n - 1);
        }
        if n % rows != rows - 1 {
            neighbors.push(n + 1);
        }
        neighbors.push(n - rows);
        neighbors.push(n + rows);

        for neighbor in neighbors {
            if seen.contains(&neighbor) {
                count += 1;
            }
        }

        if count >= 2 {
            return false;
        }

        seen.insert(n);
    }
    true
}

fn generate_longest_path(rows: usize, cols: usize) -> Vec<Vec<petgraph::prelude::NodeIndex>> {
    let mut graph = Graph::new_undirected();

    let mut nodes = vec![];

    for i in 0..rows {
        for j in 0..cols {
            let node = graph.add_node(i + j * cols);
            nodes.push(node);
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            let node = nodes[i + j * rows];
            if i != rows - 1 {
                let next = nodes[(i + 1) + j * rows];
                graph.add_edge(node, next, 1);
            }
            if j != cols - 1 {
                let next = nodes[i + (j + 1) * rows];
                graph.add_edge(node, next, 1);
            }
        }
    }

    let mut ways: Vec<Vec<petgraph::prelude::NodeIndex>> = Vec::new();

    for i in 0..(rows * cols) {
        let max = ways.iter().map(|x| x.len()).max().unwrap_or_default();
        for j in (i + 1)..(rows * cols) {
            let mut found =
                algo::all_simple_paths::<Vec<_>, _>(&graph, nodes[i], nodes[j], 0, None)
                    .filter(|x| x.len() > max)
                    .filter(|x| verify(x, rows as i32, cols as i32))
                    .collect::<Vec<_>>();
            ways.append(&mut found);
        }
    }

    let max = ways.iter().map(|x| x.len()).max().unwrap();
    let longest: Vec<Vec<petgraph::prelude::NodeIndex>> =
        ways.into_iter().filter(|x| x.len() == max).collect();

    longest
}

fn generate_image(path: Vec<petgraph::prelude::NodeIndex>, rows: u32, cols: u32) -> RgbImage {
    let tile_height = 45;
    let tile_width = 45;
    let mut img = RgbImage::new(tile_height * rows, tile_width * cols);
    let empty_tile = image::open("tiles/water.png").unwrap().into_rgb8();
    let set_tile = image::open("tiles/tile.png").unwrap().into_rgb8();

    let set_tiles: HashSet<usize> = HashSet::from_iter(path.iter().map(|x| x.index()));

    for i in 0..rows {
        for j in 0..cols {
            let path_index = (i + rows * j) as usize;
            if set_tiles.contains(&path_index) {
                img.copy_from(&set_tile, i * tile_height, j * tile_width)
                    .expect("could not copy sub-image");
            } else {
                img.copy_from(&empty_tile, i * tile_height, j * tile_width)
                    .expect("could not copy sub-image");
            }
        }
    }
    img
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: ./{} rows cols", args[0]);
        return;
    }
    let rows = args[1].parse::<usize>().unwrap();
    let cols = args[2].parse::<usize>().unwrap();
    let paths = generate_longest_path(rows, cols);
    for i in 0..paths.len() {
        let img = generate_image(paths[i].clone(), rows as u32,cols as u32);
        let name = format!("output/{}x{}_{}.png", rows, cols, i);
        img.save(name).expect("could not write image");
    }
}

#[test]
fn test_longest_paths() {
    assert_eq!(generate_longest_path(3, 3).len(), 8);
    assert_eq!(generate_longest_path(3, 4).len(), 14);
    assert_eq!(generate_longest_path(4, 4).len(), 84);
}

#[test]
fn test_verify() {
    let mut a: Vec<petgraph::prelude::NodeIndex<u32>> = [
        NodeIndex::new(0),
        NodeIndex::new(3),
        NodeIndex::new(6),
        NodeIndex::new(7),
        NodeIndex::new(8),
    ]
    .to_vec();
    assert!(verify(&a, 3, 3));

    a.push(NodeIndex::new(5));
    assert!(verify(&a, 3, 3));

    let mut a: Vec<petgraph::prelude::NodeIndex<u32>> = [
        NodeIndex::new(0),
        NodeIndex::new(1),
        NodeIndex::new(2),
        NodeIndex::new(5),
        NodeIndex::new(8),
        NodeIndex::new(7),
        NodeIndex::new(10),
    ]
    .to_vec();
    assert!(verify(&a, 3, 4));

    a.push(NodeIndex::new(9));
    dbg!(&a);
    assert!(verify(&a, 3, 4));
}
