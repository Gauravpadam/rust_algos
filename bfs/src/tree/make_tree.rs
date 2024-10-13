use rand::seq::SliceRandom;

use super::node::Node;

pub fn tree_builder(data_points: i32) -> Option<Node>{

    let mut rng = rand::thread_rng();

    let mut dp: Vec<i32> = (1..(data_points + 1)).collect();
    dp.shuffle(&mut rng);

    let tree: Option<Node> = helper(&dp, 0, dp.len() as i32);

    tree
}

fn helper(elements: &Vec<i32>, i: usize, size: i32) -> Option<Node>{
    if i >= size as usize{
        return None;
    }

    let mut root: Node = Node::new(elements[i]);
    root.left = helper(elements, 2 * i + 1, size).map(Box::new);
    root.right = helper(elements, 2 * i + 2, size).map(Box::new);

    Some(root)

}