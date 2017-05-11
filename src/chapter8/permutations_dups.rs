use std::collections::HashMap;

pub fn permutations(s: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut table = build_freq_table(s);
    let mut result = vec![];
    get_perms(&mut table, vec![], s.len() as i32, &mut result);

    result
}

fn build_freq_table(s: &Vec<u8>) -> HashMap<u8, i32> {
    let mut freq_table = HashMap::new();
    for byte in s.iter() {
        if !freq_table.contains_key(byte) {
            freq_table.insert(*byte, 1);
        } else {
            *freq_table.get_mut(byte).unwrap() += 1;
        }
    }

    freq_table
}

fn get_perms(freq_table: &mut HashMap<u8, i32>,
             prefix: Vec<u8>,
             remaining: i32,
             result: &mut Vec<Vec<u8>>) {
    if remaining == 0 {
        result.push(prefix);
        return;
    }

    for (byte, count) in freq_table.clone().iter() {
        if *count > 0 {
            *freq_table.get_mut(byte).unwrap() -= 1;

            let mut new_prefix = prefix.clone();
            new_prefix.push(*byte);
            get_perms(freq_table, new_prefix, remaining - 1, result);

            *freq_table.get_mut(byte).unwrap() = *count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permutations_dups() {
        let s = "Hello".to_string();
        let perms = permutations(&mut s.into_bytes());
        let perms_str: Vec<_> =
            perms.into_iter().map(|perm| String::from_utf8(perm).unwrap()).collect();
        let mut set = HashSet::new();

        for perm in perms_str {
            assert!(!set.contains(&perm));

            set.insert(perm);
        }
    }
}
