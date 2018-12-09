use std::error;
use std::fmt;

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
}

