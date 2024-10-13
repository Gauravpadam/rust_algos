pub struct Node{
    pub data: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}

impl Node{
    pub fn new(data:i32) -> Self{
        Node{
            data,
            left: None,
            right: None
        }
    }
}