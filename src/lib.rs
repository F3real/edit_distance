#![crate_name = "edit_distance"]
#![crate_type = "lib"]

use std::cmp;

pub fn edit_distance(str1: &str, str2: &str) -> u16 {

    // we add one extra since first element in matrix is 0 which doesn't correspond to any letter
    let str1_len: usize = str1.len() + 1;
    let str2_len: usize = str2.len() + 1;

    let mut res_array: Vec<Vec<u16>> = (0..str1_len).map(|_| vec![0; str2_len]).collect();
   
    //initialize starting values for edit distance
    for i in 0..str1_len {
       res_array[i][0] = i as u16; 
    }

    for i in 0..str2_len {
       res_array[0][i] = i as u16; 
    }

    //calcuate distance
    for i in 1..str1_len {
        for j in 1..str2_len {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                res_array[i][j] = res_array[i - 1][j - 1];
            }
            else {
                res_array[i][j] = cmp::min(
                    cmp::min(res_array[i - 1][j - 1], res_array[i - 1][j]),
                    res_array[i][j - 1]) + 1;
            }
        }
    }

    res_array[str1_len - 1][str2_len - 1]
}

#[cfg(test)]
mod tests {
    use super::edit_distance;

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("test", "tast"), 1);
        assert_eq!(edit_distance("taming text", "tamming test"), 2);
        assert_eq!(edit_distance("Zeil", "trials"), 4);
    }
}