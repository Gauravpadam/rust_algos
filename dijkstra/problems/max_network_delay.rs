use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Logic: Dijkstra on a proper graph with nodes

/*
This is similar to dijkstra on grids, a few logical changes where we don't use a two dimensional array to update the
distances, rather a 1D array can do the same

Always remember the logic to create adjacency lists as it is the most important thing in graph questions

That's it, cool
*/

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {

        // let m = times.len();
        // let n = times[0].len();
        // let mut distances = vec![vec![i32::MAX; n]; m];
        // let mut heap = BinaryHeap::new();
        // let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        // distances[k as usize][k as usize] = 0;

        // heap.push(Reverse((k, k, 0)));

        // while !heap.is_empty(){
        //     if let Some(Reverse((r, c, effort))) = heap.pop(){

        //         if r == k && c == k{
        //             return effort;
        //         }

        //         for (dr, dc) in &directions{
        //             let new_r = r as isize + dr;
        //             let new_c = c as isize + dc;

        //             if new_r >= 0 && new_r < m as isize && new_c >= 0 && new_c < n as isize{
        //                 let mut new_effort = (times[r as usize][c as usize] - times[new_r][new_c]).abs().max(effort);

        //                 if new_effort < effort{
        //                     distances[new_r][new_c] = new_effort;
        //                     heap.push(Reverse((new_r, new_c, new_effort)))
        //                 }

        //             }

        //         }
                
        //     }

        //     4
        // }

        /* This is not a grid based sum, It's a graph based sum so construct a graph adjacency list for it */

        let mut graph = vec![vec![]; n as usize];

        // Creating an adjacency list in graph based sums
        for time in times{
            let u = time[0] as usize - 1;
            let v = time[1] as usize - 1;
            let w = time[2];
            graph[u].push((v, w));
        }

        // Creating a distance array as 1D in graph based approach
        let mut distances: Vec<i32> = vec![i32::MAX; n as usize];

        // Distance to source = 0
        distances[(k - 1) as usize] = 0;

       // Binary heap for graph sums
       let mut heap = BinaryHeap::new();
       heap.push(Reverse((0, (k - 1) as usize)));

       while let Some(Reverse((time, node))) = heap.pop(){

        if time > distances[node]{
            continue;
        }

        // Iterate the adjacency list
        for &(neighbour, weight) in &graph[node]{
            // new time calculation for the adjacent nodes
            let new_time = time + weight;

            // updation step
            if new_time < distances[neighbour]{
                distances[neighbour] = new_time;
                heap.push(Reverse((new_time, neighbour)));
            }
        }

       }

       // Maximum network delay is the maximum distance spotted in the entire iteration
       let max_dist = *distances.iter().max().unwrap();

       // Some nodes might be unreachable, we don't consider them in this network
       if max_dist == i32::MAX{
        -1
       } else{
        max_dist
       }
    

    }
}