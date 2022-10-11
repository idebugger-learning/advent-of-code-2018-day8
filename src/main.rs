fn main() {
    let input = include_str!("./data/input.txt");
    // let input = include_str!("./data/input_example.txt");

    let input = input
        .split(' ')
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let (sum, _) = sum_of_metadata(&input, 0);

    println!("Sum of metadata: {}", sum);
}

fn sum_of_metadata(input: &Vec<u64>, start: usize) -> (u64, u64) {
    let children_len = input[start];
    let metadata_len = input[start + 1];

    let mut sum = 0;
    let mut children_length = 0;
    for _ in 0..children_len {
        let (sum_of_children, child_len) =
            sum_of_metadata(input, start + 2 + children_length as usize);
        children_length += child_len;
        sum += sum_of_children;
    }

    for i in 0..metadata_len {
        sum += input[start + 2 + children_length as usize + i as usize];
    }

    (sum, children_length + 2 + metadata_len)
}
