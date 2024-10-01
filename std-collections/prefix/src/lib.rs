#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    let mut v_iter: Vec<_> = strs.clone().into_iter().map(|s| s.chars()).collect();
    let mut s_res = String::from("");

    if v_iter.len() == 0 {
        return s_res;
    }

    for j in 0..strs[0].len() {
        let mut flag = true;
        let c = v_iter[0].next().unwrap();
        for i in 1..v_iter.len() {
            match (c, v_iter[i].next()) {
                (c1, Some(c2)) => {
                    if c1 != c2 {
                        flag = false;
                    }
                },
                (_, _) => flag = false,
            }
        }

        if flag {
            print!("{:?} ", c);
            s_res.push(c);
        } else {
            break;
        }
    }
    
    s_res
}
