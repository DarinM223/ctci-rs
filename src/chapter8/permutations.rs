pub fn permutations(s: &mut Vec<u8>) -> Vec<Vec<u8>> {
    if s.is_empty() {
        return vec![vec![]];
    }

    let curr_char = s.remove(0);
    let prev_perm = permutations(s);
    let mut new_perm = vec![];

    for permutation in prev_perm.iter() {
        append_to_permutation(permutation, curr_char, &mut new_perm);
    }

    new_perm
}

fn append_to_permutation(permutation: &[u8], ch: u8, appended_perms: &mut Vec<Vec<u8>>) {
    for pos in 0..=permutation.len() {
        let mut permutation = permutation.to_owned();
        permutation.insert(pos, ch);
        appended_perms.push(permutation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permutations() {
        let s = "Blah".to_string();
        let perms = permutations(&mut s.into_bytes());
        let perms_str: Vec<_> = perms
            .into_iter()
            .map(|perm| String::from_utf8(perm).unwrap())
            .collect();
        let mut set = HashSet::new();

        for perm in perms_str {
            assert!(!set.contains(&perm));

            set.insert(perm);
        }
    }
}
