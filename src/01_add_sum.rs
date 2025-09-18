impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut vc:Vec<i32> = Vec::new();

        for (i, e1) in nums.iter().enumerate() {

            for (j, e2) in nums.iter().enumerate() {
                if i != j && e1 + e2 == target {
                    vc.push(i as i32);
                    vc.push(j as i32);
                    return vc;
                }
            }
        }

        vc
    }
}
