use tree::{make_tree::tree_builder, node::Node};
mod tree;
use std::collections::VecDeque;

fn main() {
    let question = tree_builder(8);

    /* Store Box<Node> instead of Node, moving box is cheaper */
    // let mut queue: VecDeque<Node> = VecDeque::new();
    // queue.push_back(question.unwrap()); 

    let mut queue: VecDeque<Box<Node>> = VecDeque::new();

    if let Some(root) = question{
        queue.push_back(Box::new(root));
    }
    let mut ans: Vec<Vec<i32>> = Vec::new();

    while !queue.is_empty(){
        let qlen = queue.len();
        let mut row: Vec<i32> = vec![];

        for i in 0..qlen{
            let curr = queue.pop_front().unwrap();
            row.push(curr.data);

            match curr.left {
                Some(_) => {
                    queue.push_back(curr.left.unwrap());
                }

                None => {}
            }


            match curr.right {
                Some(_) => {
                    queue.push_back(curr.right.unwrap());
                }

                None => {}
            }

        }
        ans.push(row);
    }

    println!("Answer for {} data points: {:?}", 8, ans);
}
