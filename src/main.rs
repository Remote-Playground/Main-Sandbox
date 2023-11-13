// Pseudo code
// i -> Current index
// difference = target - nums[i]
// return value -> temp[difference], i]
use std::collections::HashMap;

fn main() {
    let mut temp: HashMap<i32, i32> = HashMap::new();

    Solution::find_target(vec![12, 3, 33, 6], 18, temp);
}

struct Solution;

impl Solution {
    fn find_target(nums: Vec<i32>, target: i32, mut temp: HashMap<i32, i32>) {
        for i in 0..nums.len() {
            let difference = target - nums[i];
            if temp.contains_key(&difference) {
                println!("{:?}", vec![temp[&difference], i.to_string().parse::<i32>().unwrap()]);
            } else {
                temp.insert(*&nums[i], i.to_string().parse::<i32>().unwrap());
            }
        }
    }
}