#![forbid(unsafe_code)]


pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }

    if arr.len() < k {
        return Vec::<Vec<i32>>::new();
    }

    if k == 1 {
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in arr {
            res.push(vec![*i]);
        }

        return res;
    }

    if k == arr.len() {
        return vec![arr.to_vec()];
    }

    let mut res: Vec<Vec<i32>> = combinations(&arr[1..], k-1);
    for i in 0..res.len() {
        res[i].insert(0, arr[0]);
    }

    res.extend(combinations(&arr[1..], k));

    return res;
}
