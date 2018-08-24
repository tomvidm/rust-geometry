mod math;

fn main() {
    let nums = vec![1,2,3,4,5,6,7,8,9];
    let doubled: Vec<i64> = nums.iter().map(|&x| 2*x - 1).collect();
    println!("{:?}", doubled);
}
