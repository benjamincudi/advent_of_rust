use std::error;
use std::fmt;

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
}

