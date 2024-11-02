pub struct DisjointSet{
    rank: Vec<i32>,
    parent: Vec<i32>,
    size: Vec<i32>
}

impl DisjointSet{

    pub fn new(n: usize) -> Self{
        DisjointSet {
            rank: vec![0; n + 1],
            parent: (0..=n).map(|x| x as i32).collect(),
            size: (0..=n).map(|x| x as i32).collect()
        }
    }

    // A method which finds ultimate parent of a node recursively (and compresses the path along it's way)
    pub fn find_u_par(&mut self, node: i32) -> i32{
        if node == self.parent[node as usize]{ // When a node is itself it's parent, we have our ultimate parent
            return node;
        }

        self.parent[node as usize] = self.find_u_par(self.parent[node as usize]); // Path compression

        self.parent[node as usize]
    }

    // A method which creates edges between graphs
    // When two graphs have same parents, they are already in union, hence we return
    // When rank of one is greater than other, we merge the smaller rank with the larger one and update the smaller's parent as the larger one
    // When ranks are same, we merge either one with another, update the parent and increase the rank by 1as the height increases by 1
    pub fn union_by_rank(&mut self, u: i32, v: i32){
        let ul_parent_u = self.find_u_par(u);
        let ul_parent_v = self.find_u_par(v);

        if ul_parent_u == ul_parent_v{
            return;
        }

        if self.rank[ul_parent_u as usize] < self.rank[ul_parent_v as usize]{
            self.parent[ul_parent_u as usize] = ul_parent_v;
        } else if self.rank[ul_parent_u as usize] > self.rank[ul_parent_v as usize]{
            self.parent[ul_parent_v as usize] = ul_parent_u;
        } else{
            // Same rank, height will increase when any one graph gets attached to another
            self.parent[ul_parent_u as usize] = ul_parent_v;
            self.rank[ul_parent_u as usize]+=1;
        }
    }

    // Size is intuitive, it indicates the heights correctly in size array
    pub fn union_by_size(&mut self, u: i32, v: i32){
        let ul_parent_u = self.find_u_par(u);
        let ul_parent_v = self.find_u_par(v);

        // both are in union
        if ul_parent_u == ul_parent_v{
            return;
        }

        if self.size[ul_parent_u as usize] < self.size[ul_parent_v as usize]{
            // < case, we attach u to v and increase size of v
            self.parent[ul_parent_u as usize] = ul_parent_v;
            self.size[ul_parent_v as usize]+=self.size[ul_parent_u as usize]; 
        } else{
            // >= case, we attach v to u and increase size of u
            self.parent[ul_parent_v as usize] = ul_parent_u;
            self.size[ul_parent_u as usize]+=self.size[ul_parent_v as usize];
        }
    }
}

