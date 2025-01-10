fn unique_norm(a: Vec<i32>) -> Vec<i32> {
    let mut deduped = a.clone();
    deduped.sort_unstable();
    deduped.dedup();
    deduped
}

// advanced 1: use generic types
fn unique_generic<T>(a: Vec<T>) -> Vec<T>
where T:Clone, T:Ord
{
    let mut deduped = a.clone();
    deduped.sort_unstable_by(|x: &T, y: &T| x.cmp(y));
    deduped.dedup();
    deduped
}

// advanced 2: keep items in order
fn unique_order<V, T>(a: V) -> Vec<T>
where V:IntoIterator<Item = T>, T:PartialEq
{
    let mut deduped: Vec<T> = Vec::new();
    for elem in a {
        if !deduped.contains(&elem) {
            deduped.push(elem);
        }
    }
    deduped
}

// advanced 3: use iterators
fn unique<V, T>(a: V) -> Vec<T>
where V:IntoIterator<Item = T>, T:PartialEq
{
    let mut deduped: Vec<T> = Vec::new();
    for elem in a {
        if !deduped.contains(&elem) {
            deduped.push(elem);
        }
    }
    deduped
}

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input:Vec<i32> = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
