
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Logic: Dijkstra on Graph

/*
Like any other graph question, dijkstra uses an adjacency list
This list will be stored in a minheap and behave like a priority queue
The essence of this algorithm lies in understanding how at each step we move to all the possible nodes
from the current node and reduce the distance from that node to the next node
(Yes, this is a travelling salesman problem. Yes, we use BFS here)
Initially everything is at infinity, but only if we keep walking...

That's it, easy
*/

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {

        // s = (0,0)
        // g = (m, n)
        // adj = []

        let rows: usize = heights.len();
        let cols: usize = heights[0].len();
        let mut heap = BinaryHeap::new();
        let mut dist = vec![vec![i32::MAX; cols]; rows]; // Everything is at infinity
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        dist[0][0] = 0; // Source to source is 0

        heap.push(Reverse((0, 0, 0))); // (effort, row, col)

        while let Some(Reverse((effort, r, c))) = heap.pop(){

            if r == rows - 1 && c == cols - 1{
                return effort; // Return the effort to reach the bottom-right corner
            }

            for (dr, dc) in &directions{
                let new_r = r as isize + dr;
                let new_c = c as isize + dc;

                if new_r >= 0 && new_r < rows as isize && new_c < cols as isize && new_c >= 0{ // Valid cell check
                    let new_r = new_r as usize;
                    let new_c = new_c as usize;

                    let mut new_effort = (heights[r][c] - heights[new_r][new_c]).abs().max(effort);

                    if new_effort < dist[new_r][new_c]{
                        dist[new_r][new_c] = new_effort;
                        heap.push(Reverse((new_effort, new_r, new_c)));
                    }
                }
            }
        }

        dist[rows - 1][cols - 1]





    }
}