struct Header {
    num_children: usize,
    metadata_count: usize,
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

