use std::fs;
use std::str::FromStr;
use std::collections::BinaryHeap;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::PartialEq;
use std::cmp::Ordering;

pub fn soln_a(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let arr: Vec<Vec<i32>> = file_str.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    
    Ok(dijkstra(&arr))
}

fn dijkstra(arr: &Vec<Vec<i32>>) -> i32 {
    let nrows = arr.len();
    let ncols = arr[0].len();
    
    let mut visited = vec![vec![false; ncols]; nrows];
    let mut to_visit: BinaryHeap<Visit<(usize, usize)>> = BinaryHeap::new();
    to_visit.push(Visit {
        vertex: (0,0),
        distance: 0
    });
    
    while let Some(Visit { vertex , distance }) = to_visit.pop() {
        let (i0,j0) = vertex;
        if visited[i0][j0] {
            continue;
        }
        if i0 == nrows - 1 && j0 == ncols - 1 {
            return distance;
        }
        visited[i0][j0] = true;
        for &(i,j) in [(i0 + 1, j0), (i0.saturating_sub(1), j0), (i0, j0 + 1), (i0, j0.saturating_sub(1))].iter() {
            if  i < nrows && j < ncols {
                to_visit.push(Visit {
                    vertex: (i,j),
                    distance: distance + arr[i][j]
                })
            }
        }
    }
    return i32::MAX;
}

pub fn soln_b(file: &str) -> Result<i32, String> {
    let file_str = match fs::read_to_string(file) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => s
    };
    let arr: Vec<Vec<i32>> = file_str.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    let nrows = arr.len();
    let ncols = arr[0].len();
    let mut arr_large: Vec<Vec<i32>> = vec![vec![0; ncols * 5]; nrows * 5];
    for i in 0..(nrows * 5) {
        for j in 0..(ncols * 5) {
            arr_large[i][j] = ((arr[i % nrows][j % ncols] + ((i / nrows) as i32) + ((j / ncols) as i32)) - 1) % 9 + 1;
        }
    }
    
    Ok(dijkstra(&arr_large))
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: i32,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}