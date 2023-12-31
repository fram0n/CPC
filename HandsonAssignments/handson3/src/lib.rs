use std::cmp;

/// by using dynamic programming we build a matrix (n+1)*(d+1)
/// so row i refers to the city i and column j refers to the days we have of vacation
/// for cell (i,j) we can:
/// not select city i --> matrix[i-1][j]
/// select city i --> for day in 1..d+1 we check every (j-day) column in the previous row and add the value corrisponding to the city i with weight=day
/// take the maximum between all these values
pub fn holiday_planning(n: usize, d: usize, values: Vec<i32>) -> i32 {
    // create a matrix with n+1 rows and d+1 columns
    let mut matrix: Vec<Vec<i32>> = vec![vec!(-1; d + 1); n + 1];
    for i in 0..n + 1 {
        for j in 0..d + 1 {
            if i == 0 || j == 0 {
                //base case
                matrix[i][j] = 0;
            } else {
                let mut max = 0;
                let mut day: i32 = 1;
                while j as i32 - day >= 0 && day < (d + 1).try_into().unwrap() {
                    max = cmp::max(
                        max,
                        matrix[i - 1][j - day as usize] + values[day as usize - 1 + (i - 1) * d],
                    );
                    day += 1;
                }
                matrix[i][j] = cmp::max(max, matrix[i - 1][j]);
            }
        }
    }
    // println!("{:?}", matrix);
    return matrix[n][d];
}

/* --------------------------------------------------------- PROBLEM 2 --------------------------------------------------------- */

/// first sort the topics in the vector by increasing beauty
/// then apply Longest Increasing Subsequence on the difficulty of the topics
pub fn design_course(n: usize, mut values: Vec<(i32, i32)>) -> usize {
    //sort the vector by increasing beauty
    values.sort_by(|a, b| a.0.cmp(&b.0));
    //LIS by checking the difficulty
    let mut res: Vec<(i32, i32)> = Vec::new();
    res.push(values[0]);
    for i in 1..n {
        if values[i].1 > res.last().unwrap().1 && values[i].0 != res.last().unwrap().0 {
            res.push(values[i]);
        } else {
            let low = lower_bound(&res, res.len(), values[i].1);
            if low == 0 || (low != 0 && res[low - 1].0 != values[i].0) {
                res[low] = values[i];
            }
        }
    }
    return res.len();
}

/// binary search to find the smallest element that is greater than or equal to value
pub fn lower_bound(res: &[(i32, i32)], mut high: usize, value: i32) -> usize {
    let mut low = 0;
    while low < high {
        let mid = low + (high - low) / 2;

        if res[mid].1 < value {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}
