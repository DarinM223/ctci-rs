pub fn permutations(s: &mut Vec<u8>) -> Vec<Vec<u8>> {
    if s.len() == 0 {
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

fn append_to_permutation(permutation: &Vec<u8>, ch: u8, appended_perms: &mut Vec<Vec<u8>>) {
    for pos in 0..permutation.len() + 1 {
        let mut permutation = permutation.clone();
        permutation.insert(pos, ch);
        appended_perms.push(permutation);
    }
}
