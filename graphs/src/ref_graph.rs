use std::cell::UnsafeCell;
use std::collections::HashSet;
use std::collections::VecDeque;
use typed_arena::Arena;

struct Node<'a> {
    name: &'static str,
    edges: UnsafeCell<Vec<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new<'b>(name: &'static str, arena: &'b Arena<Node<'b>>) -> &'b Node<'b> {
        arena.alloc(Node {
            name: name,
            edges: UnsafeCell::new(Vec::new()),
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
        unsafe {
            for n in &(*self.edges.get()) {
                n.dfs_traverse(f, seen);
            }
        }
    }

    fn bfs_traverse<F>(&'a self, f: &F, seen: &mut HashSet<&'static str>)
    where
        F: Fn(&'static str),
    {
        let mut visit_queue: VecDeque<&'a Node> = VecDeque::new();
        visit_queue.push_back(self);

        loop {
            match visit_queue.pop_front() {
                None => break,
                Some(node) => {
                    if !seen.contains(node.name) {
                        f(node.name);
                        seen.insert(node.name);
                        unsafe {
                            for n in &(*node.edges.get()) {
                                if !seen.contains(n.name) {
                                    visit_queue.push_back(n);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn first(&'a self) -> &'a Node<'a> {
        unsafe { (*self.edges.get())[0] }
    }
}

fn foo<'a>(node: &'a Node<'a>) {
    println!("foo: {}", node.name);
}

fn init<'a>(arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
    let root = Node::new("A", arena);

    let b = Node::new("B", arena);
    let c = Node::new("C", arena);
    let d = Node::new("D", arena);
    let e = Node::new("E", arena);
    let f = Node::new("F", arena);

    unsafe {
        (*root.edges.get()).push(b);
        (*root.edges.get()).push(c);
        (*root.edges.get()).push(d);

        (*c.edges.get()).push(e);
        (*c.edges.get()).push(f);
        (*c.edges.get()).push(root);
    }

    root
}

pub fn main() {
    let arena = Arena::new();
    let g = init(&arena);
    g.dfs_traverse(&|d| print!("{} -> ", d), &mut HashSet::new());
    println!("");
    g.bfs_traverse(&|d| print!("{} -> ", d), &mut HashSet::new());
    println!("");
    foo(g.first());
}
