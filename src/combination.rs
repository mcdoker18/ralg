// https://leetcode.com/problems/combination-sum-ii/description/
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates.clone();
    candidates.sort();

    let mut res = Vec::new();

    let mut pathes: Vec<i32> = Vec::new();

    dfs_combination_sum2(&candidates, &mut res, &mut pathes, target);

    res
}

fn dfs_combination_sum2(
    candidates: &[i32],
    res: &mut Vec<Vec<i32>>,
    pathes: &mut Vec<i32>,
    sum: i32,
) {
    if sum < 0 {
        return;
    }

    if sum == 0 {
        res.push(pathes.clone());

        return;
    }

    if candidates.is_empty() {
        return;
    }

    if sum < candidates[0] {
        return;
    }

    for (i, elem) in candidates.iter().enumerate() {
        if i != 0 && candidates[i] == candidates[i - 1] {
            continue;
        }

        pathes.push(*elem);

        dfs_combination_sum2(&candidates[i + 1..], res, pathes, sum - elem);

        pathes.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(combination_sum2(Vec::new(), 3).is_empty());
    }

    #[test]
    fn example_1() {
        assert_eq!(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 2, 2], vec![5]],
            combination_sum2(vec![2, 5, 2, 1, 2], 5),
        );
    }

    #[test]
    fn not_found() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            combination_sum2(vec![2, 5, 2, 1, 2], 99),
        );
    }
}
