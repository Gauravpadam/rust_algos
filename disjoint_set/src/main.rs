mod Disjoint_set;

use Disjoint_set::DisjointSet;

fn main(){
    let mut first_set = DisjointSet::new(7);
    first_set.union_by_rank(1, 2);
    first_set.union_by_rank(2, 3);
    first_set.union_by_rank(4, 5);
    first_set.union_by_rank(6, 7);
    first_set.union_by_rank(5, 6);

    if first_set.find_u_par(3) == first_set.find_u_par(7){
        println!("Same");
    } else{
        println!("Not same");
    }

    first_set.union_by_rank(3, 7);

    if first_set.find_u_par(3) == first_set.find_u_par(7){
        println!("Now same");
    } else{
        println!("Still not same")
    }

    let mut second_set = DisjointSet::new(7);
    second_set.union_by_size(1, 2);
    second_set.union_by_size(2, 3);
    second_set.union_by_size(4, 5);
    second_set.union_by_size(6, 7);
    second_set.union_by_size(5, 6);

    if second_set.find_u_par(3) == second_set.find_u_par(7){
        println!("Same");
    } else{
        println!("Not same");
    }

    second_set.union_by_size(3, 7);

    if second_set.find_u_par(3) == second_set.find_u_par(7){
        println!("Now same");
    } else{
        println!("Still not same")
    }
}