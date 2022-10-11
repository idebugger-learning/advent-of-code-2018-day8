struct Node {
    metadata: Vec<u64>,
    children: Vec<Box<Node>>,
    value: u64,
    length: usize,
}

impl Node {
    fn new(input: &[u64]) -> Box<Self> {
        let children_count = input[0];
        let metadata_count = input[1];

        let mut own_length = 2;
        let mut children = vec![];
        for _ in 0..children_count {
            let node = Node::new(&input[own_length..]);
            own_length += node.length;
            children.push(node);
        }

        let mut metadata = vec![];
        for _ in 0..metadata_count {
            metadata.push(input[own_length]);
            own_length += 1;
        }

        let value = if children_count == 0 {
            metadata.iter().sum::<u64>()
        } else {
            metadata
                .iter()
                .filter(|&idx| idx > &0)
                .filter(|&idx| idx - 1 < children_count)
                .map(|&idx| &children[idx as usize - 1].value)
                .sum::<u64>()
        };

        Box::new(Node {
            metadata,
            children,
            value,
            length: own_length,
        })
    }
}

fn main() {
    let input = include_str!("./data/input.txt");
    // let input = include_str!("./data/input_example.txt");

    let input = input
        .split(' ')
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let (sum, _) = sum_of_metadata(&input, 0);

    println!("Sum of metadata: {}", sum);

    let root_node = Node::new(&input);
    println!("Value of root node {}", root_node.value);
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
