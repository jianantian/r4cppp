use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;
use typed_arena::Arena;

pub struct Node<'a> {
    name: &'static str,
    edges: RefCell<Vec<&'a Node<'a>>>,
}

pub struct Graph<'a> {
    vertices: RefCell<Vec<&'a Node<'a>>>,
    names: RefCell<HashSet<&'static str>>,
    edge_num: RefCell<usize>,
}

impl<'a> Node<'a> {
    fn new<'b>(name: &'static str, arena: &'b Arena<Node<'b>>) -> &'b Node<'b> {
        arena.alloc(Node {
            name: name,
            edges: RefCell::new(Vec::new()),
        })
    }

    fn dfs_traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
    where
        F: Fn(&'static str),
    {
        if seen.contains(&self.name) {
            return;
        }
        f(self.name);
        seen.insert(self.name);
        for n in self.edges.borrow().iter() {
            n.dfs_traverse(f, seen);
        }
    }

    fn bfs_traverse<F>(&'a self, f: &F, seen: &mut HashSet<&'static str>)
    where
        F: Fn(&'static str),
    {
        let mut visit_queue: VecDeque<&'a Node<'a>> = VecDeque::new();
        visit_queue.push_back(self);
        loop {
            match visit_queue.pop_front() {
                None => break,
                Some(node) => {
                    if !seen.contains(node.name) {
                        f(node.name);
                        seen.insert(node.name);
                        for n in node.edges.borrow().iter() {
                            if !seen.contains(n.name) {
                                visit_queue.push_back(n);
                            }
                        }
                    }
                }
            }
        }
    }

    // fn first(&'a self) -> &'a Node<'a> {
    //     self.edges.borrow()[0]
    // }
}

impl<'a> Graph<'a> {
    pub fn new<'b>(arena: &'b Arena<Graph<'b>>) -> &'b Graph<'b> {
        arena.alloc(Graph {
            vertices: RefCell::new(Vec::new()),
            names: RefCell::new(HashSet::new()),
            edge_num: RefCell::new(0),
        })
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.borrow().len()
    }

    pub fn num_edges(&self) -> usize {
        *self.edge_num.borrow()
    }

    fn add_vertices(&'a self, node: &'a Node<'a>) {
        self.vertices.borrow_mut().push(node);
        self.names.borrow_mut().insert(&node.name);
    }

    pub fn add_edge(&'a self, start: &'a Node<'a>, end: &'a Node<'a>) {
        start.edges.borrow_mut().push(end);
        *self.edge_num.borrow_mut() += 1;
        if !self.names.borrow().contains(start.name) {
            self.add_vertices(start);
        }
        if !self.names.borrow().contains(end.name) {
            self.add_vertices(end);
        }
    }

    pub fn dfs_traverse<F>(&self, f: &F)
    where
        F: Fn(&'static str),
    {
        let length = self.num_vertices();
        let mut seen: HashSet<&'static str> = HashSet::new();
        for node in self.vertices.borrow().iter() {
            node.dfs_traverse(f, &mut seen);
            if seen.len() >= length {
                return;
            }
        }
    }

    pub fn bfs_traverse<F>(&self, f: &F)
    where
        F: Fn(&'static str),
    {
        let length = self.num_vertices();
        let mut seen: HashSet<&'static str> = HashSet::new();
        for node in self.vertices.borrow().iter() {
            node.bfs_traverse(f, &mut seen);
            if seen.len() >= length {
                return;
            }
        }
    }
}

// fn foo<'a>(node: &'a Node<'a>) {
//     println!("foo: {}", node.name);
// }

// fn init<'a>(arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
//     let root = Node::new("A", arena);

//     let b = Node::new("B", arena);
//     let c = Node::new("C", arena);
//     let d = Node::new("D", arena);
//     let e = Node::new("E", arena);
//     let f = Node::new("F", arena);

//     root.edges.borrow_mut().push(b);
//     root.edges.borrow_mut().push(c);
//     root.edges.borrow_mut().push(d);

//     c.edges.borrow_mut().push(e);
//     c.edges.borrow_mut().push(f);
//     c.edges.borrow_mut().push(root);

//     root
// }

fn init_graph<'a>(
    arena_graph: &'a Arena<Graph<'a>>,
    arena_node: &'a Arena<Node<'a>>,
) -> &'a Graph<'a> {
    let a = Node::new("A", arena_node);
    let b = Node::new("B", arena_node);
    let c = Node::new("C", arena_node);
    let d = Node::new("D", arena_node);
    let e = Node::new("E", arena_node);
    let f = Node::new("F", arena_node);

    let graph = Graph::new(&arena_graph);

    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(a, d);
    graph.add_edge(c, e);
    graph.add_edge(c, f);
    graph.add_edge(c, a);
    graph.add_edge(d, e);
    graph
}

pub fn main() {
    let arena_node = Arena::new();
    let arena_graph = Arena::new();
    let g = init_graph(&arena_graph, &arena_node);
    println!(
        "graph num vertices: {}, num edges: {}",
        g.num_vertices(),
        g.num_edges()
    );
    g.dfs_traverse(&|d| print!("{} -> ", d));
    println!("");
    g.bfs_traverse(&|d| print!("{} -> ", d));
    // g.dfs_traverse(&|d| print!("{} -> ", d), &mut HashSet::new());
    // println!("");
    // g.bfs_traverse(&|d| print!("{} -> ", d), &mut HashSet::new());
    // println!("");
    // foo(g.first());
}
