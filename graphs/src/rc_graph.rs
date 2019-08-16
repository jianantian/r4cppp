use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

struct Node {
    name: &'static str,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &'static str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            name: name,
            edges: Vec::new(),
        }))
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
        for n in &self.edges {
            n.borrow().dfs_traverse(f, seen);
        }
    }

    // 坑在于 self 和临近的节点不是一个类型
    fn bfs_traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
    where
        F: Fn(&'static str),
    {
        let mut visit_queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();

        f(self.name);
        seen.insert(self.name);
        for n in &self.edges {
            if !seen.contains(&n.borrow().name) {
                visit_queue.push_back(n.clone());
            }
        }

        loop {
            match visit_queue.pop_front() {
                None => break,
                Some(node) => {
                    let v_node = node.borrow();
                    f(v_node.name);
                    seen.insert(v_node.name);
                    for n in &v_node.edges {
                        if !seen.contains(&n.borrow().name) {
                            visit_queue.push_back(n.clone());
                        }
                    }
                }
            }
        }
    }

    fn first(&self) -> Rc<RefCell<Node>> {
        self.edges[0].clone()
    }
}

fn foo(node: &Node) {
    println!("foo: {}", node.name);
}

fn init() -> Rc<RefCell<Node>> {
    let root = Node::new("A");

    let b = Node::new("B");
    let c = Node::new("C");
    let d = Node::new("D");
    let e = Node::new("E");
    let f = Node::new("F");

    {
        let mut mut_root = root.borrow_mut();
        mut_root.edges.push(b.clone());
        mut_root.edges.push(c.clone());
        mut_root.edges.push(d.clone());

        let mut mut_c = c.borrow_mut();
        mut_c.edges.push(e.clone());
        mut_c.edges.push(f.clone());
        mut_c.edges.push(root.clone());
    }

    root
}

pub fn main() {
    let g = init();
    let g = g.borrow();
    g.dfs_traverse(&|d| print!("{} -> ", d), &mut HashSet::new());
    println!("");
    g.bfs_traverse(&|d| print!("{} -> ", d), &mut HashSet::new());
    let f = g.first();
    println!("");
    foo(&*f.borrow());
}
