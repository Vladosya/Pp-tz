type Identifier = u32;

#[derive(Debug)]
pub struct Node<T> {
    pub id: Identifier,
    pub val: T,
}

impl<T> Node<T>
    where
        T: std::fmt::Display + std::str::FromStr,
{
    pub fn new(id: Identifier, val: T) -> Self {
        Self { id, val }
    }

    pub fn ser(&self) -> String {
        format!("{} {}", self.id, self.val)
    }

    pub fn des(s: &str) -> Self {
        let spl: Vec<&str> = s.split(' ').collect::<Vec<&str>>();
        if spl.len() != 2 {
            panic!("parsing error of \"{}\"", s);
        } else {
            let id: Identifier = spl[0]
                .parse()
                .expect(&format!("wrong id in \"{}\"", s).to_owned());
            match spl[1].parse() {
                Ok(val) => Self { id, val },
                Err(_) => panic!("can't parse val in \"{}\"", s),
            }
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    begin: Identifier,
    end: Identifier,
}

impl Edge {
    pub fn new(begin: Identifier, end: Identifier) -> Self {
        Self { begin, end }
    }

    pub fn ser(&self) -> String {
        format!("{} {}", self.begin, self.end)
    }

    pub fn des(s: &str) -> Self {
        let spl: Vec<&str> = s.split(' ').collect::<Vec<&str>>();
        if spl.len() != 2 {
            panic!("parsing error of \"{}\"", s);
        }
        let err: String = format!("parsing error in \"{}\"", s);
        Self {
            begin: spl[0].parse().expect(&err),
            end: spl[1].parse().expect(&err),
        }
    }
}

#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

impl<T> Graph<T>
    where
        T: std::fmt::Display + std::str::FromStr,
{
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node_from(mut self, node: Node<T>) -> Self {
        if self.nodes.iter().any(|n| n.id == node.id) {
            panic!("duplicate node with id {}", node.id);
        }
        self.nodes.push(node);
        self
    }

    pub fn add_node(self, id: Identifier, val: T) -> Self {
        self.add_node_from(Node::new(id, val))
    }

    pub fn remove_node_by_id(mut self, node_id: Identifier) -> Self {
        self.edges
            .retain(|e| !(node_id == e.begin || node_id == e.end));
        self.nodes.retain(|n| n.id != node_id);
        self
    }

    pub fn add_edge_from(mut self, edge: Edge) -> Self {
        if let None = self
            .nodes
            .iter()
            .find(|n| edge.begin == n.id)
            .and(self.nodes.iter().find(|n| edge.end == n.id))
        {
            panic!(
                "adding edge between non-existent nodes ({}->{})",
                edge.begin, edge.end
            );
        }
        if !self
            .edges
            .iter()
            .any(|e| e.begin == edge.begin && e.end == edge.end)
        {
            self.edges.push(edge);
        }
        self
    }

    pub fn add_edge(self, begin: Identifier, end: Identifier) -> Self {
        self.add_edge_from(Edge::new(begin, end))
    }

    pub fn remove_edge(mut self, edge: &Edge) -> Self {
        self.edges
            .retain(|e| e.begin != edge.begin && e.end == edge.end);
        self
    }

    pub fn ser(&self) -> String {
        let mut res: String = String::new();

        for node in &self.nodes {
            res.push_str(&node.ser());
            res.push('\n');
        }
        res.push_str("#\n");
        for edge in &self.edges {
            res.push_str(&edge.ser());
            res.push('\n');
        }
        res
    }

    pub fn des(s: &str) -> Self {
        let mut res: Graph<T> = Self::new();
        let mut b: bool = true; // now parsing nodes

        for line in s.lines() {
            if line == "#" {
                b = false;
                continue;
            }
            if b {
                res = res.add_node_from(Node::des(&line));
            } else {
                res = res.add_edge_from(Edge::des(&line));
            }
        }

        res
    }

    pub fn get_connected<'a>(&'a self, node: &'a Node<T>) -> Vec<&'a Node<T>> {
        let mut res: Vec<&Node<T>> = vec![];

        for edge in &self.edges {
            if edge.begin == node.id {
                if let Some(n) = self.nodes.iter().find(|nn| nn.id == edge.end) {
                    res.push(n);
                }
            }
        }

        res
    }

    pub fn get_all_nodes(&self) -> Vec<&Node<T>> {
        self.nodes.iter().collect()
    }

    pub fn traverse_from<F>(&self, root: &Node<T>, f: &mut F)
        where
            F: FnMut(&Node<T>),
    {
        self.traverse_from_with_memory(root, f, &vec![]);
    }

    fn traverse_from_with_memory<'a, F>(
        &'a self,
        root: &'a Node<T>,
        f: &mut F,
        memory: &'a Vec<&'a Node<T>>,
    ) where
        F: FnMut(&Node<T>),
    {
        if let Some(_) = memory.iter().find(|n| n.id == root.id) {
            return;
        }
        f(root);
        let mut memory: Vec<&Node<T>> = memory.clone();
        memory.push(root);
        let nexts: Vec<&Node<T>> = self.get_connected(root);
        for next in nexts {
            self.traverse_from_with_memory(next, f, &memory);
        }
    }
}
