struct Solution;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
//         // 2 <= nums.length <= 104
//         if nums.len() < 2 {
//             return Vec::<i32>::new();
//         }

//         if nums.len() > 10_000 {
//             return Vec::<i32>::new();
//         }

//         let mut vc:Vec<i32> = Vec::new();

//         for (i, e1) in nums.iter().enumerate() {

//             for (j, e2) in nums.iter().enumerate() {
//                 if i != j && e1 + e2 == target {
//                     vc.push(i as i32);
//                     vc.push(j as i32);
//                     return vc;
//                 }
//             }
//         }

//         vc
//     }
// }

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        use std::collections::HashMap;

        let mut map = HashMap::new();

        for (ind, el)  in nums.iter().enumerate() {
            map.insert(ind, el);
        }

       for (ind, el)  in nums.iter().enumerate() {
          let complement = (target - el) as usize;


          let el_cpm = map.get(&complement).unwrap();
          if map.contains_key(&complement) && (**el_cpm != (ind as i32) ) {
            return vec![ind as i32, **el_cpm];
          }
        }

        vec![]
    }
}

fn main() {

    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![5, 1, 8, 2], 10), vec![2, 3]);
    assert_eq!(Solution::two_sum(vec![1, 5, 3], 4), [0, 2]);
    assert_eq!(Solution::two_sum(vec![1, 2, 3], 100), Vec::<i32>::new());

      // single element
    assert_eq!(Solution::two_sum(vec![42], 42), Vec::<i32>::new());
        // two elements that match
    assert_eq!(Solution::two_sum(vec![1, 2], 3), vec![0, 1]);
        // two elements that don't match
    assert_eq!(Solution::two_sum(vec![1, 2], 5), Vec::<i32>::new());

}
