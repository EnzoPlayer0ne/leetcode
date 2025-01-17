use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();

        for n in nums {
            if !map.insert(n){
                return true;
            }
        }

        false
    }
}
