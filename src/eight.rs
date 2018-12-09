use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
struct Header {
    num_children: usize,
    metadata_count: usize,
}

#[derive(Debug)]
struct NodeParseError;
impl fmt::Display for NodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid Node string")
    }
}
impl error::Error for NodeParseError {
    fn description(&self) -> &str {
        "invalid Node"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Clone)]
struct Node {
    header: Header,
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn append_child(&mut self, n: Node) {
        self.children.push(n);
    }

    fn append_metadata(&mut self, m: usize) {
        self.metadata.push(m);
    }

    fn from_vec_usize(i: &mut Vec<usize>) -> Result<Self, NodeParseError> {
        let self_header_data = vec![i.remove(0), i.remove(0)];

        let header = Header {
            num_children: self_header_data[0],
            metadata_count: self_header_data[1],
        };

        let mut node = Node {
            header: header.clone(),
            children: vec![],
            metadata: vec![],
        };

        let mut remaining_children = header.num_children.clone();
        while &remaining_children > &0 {
            match Node::from_vec_usize(i) {
                Ok(n) => node.append_child(n),
                Err(_) => panic!("failed to parse"),
            };
            remaining_children -= 1;
        }

        while node.metadata.len() < node.header.metadata_count {
            node.append_metadata(i.remove(0));
        }

        return Ok(node);
    }

    fn sum_metadata(self) -> usize {
        let mut total: usize = self.metadata.into_iter().fold(0, |acc, m| acc + m);
        for c in self.children {
            total += c.sum_metadata();
        }
        return total;
    }

    fn sum_tree(self) -> usize {
        let mut total: usize = 0;
        if self.header.num_children > 0 {
            for m in self.metadata.into_iter() {
                total += match self.children.as_slice().get(m - 1) {
                    None => 0,
                    Some(n) => n.clone().sum_tree(),
                }
            }
        } else {
            total += self.metadata.into_iter().sum::<usize>();
        };
        return total;
    }
}

impl FromStr for Node {
    type Err = NodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data_chunks: Vec<usize> = s
            .to_string()
            .as_mut_str()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        return Node::from_vec_usize(&mut data_chunks);
    }
}

pub fn part_one(file_contents: &String) -> () {
    let root: Node = file_contents.clone().as_str().parse::<Node>().unwrap();

    println!("metadata sum is {}", root.sum_metadata());
}

pub fn part_two(file_contents: &String) -> () {
    let root: Node = file_contents.clone().as_str().parse::<Node>().unwrap();

    println!("tree sum is {}", root.sum_tree());
}
