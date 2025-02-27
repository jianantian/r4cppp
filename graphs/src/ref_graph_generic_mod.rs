// Note that this one is hypothetical future Rust and will not compile today.

use std::cell::UnsafeCell;
use std::collections::HashSet;
use typed_arena::Arena;

// Module is parameterised with the lifetime of the graph.
mod graph<'a> {
    struct Node {
        name: &'static str,
        // The module-level lifetime is used for the lifetime of each Node.
        edges: UnsafeCell<Vec<&'a Node>>,
    }

    impl Node {
        fn new(name: &'static str, arena: &'a Arena<Node>) -> &'a Node {
            arena.alloc(Node {
                name: name,
                edges: UnsafeCell::new(Vec::new()),
            })
        }

        fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
            where F: Fn(&'static str)
        {
            if seen.contains(&self.name) {
                return;
            }
            f(self.name);
            seen.insert(self.name);
            for n in &self.edges {
                unsafe {
                    for n in &(*self.edges.get()) {
                        n.traverse(f, seen);
                    }
                }
            }
        }

        fn first(&self) -> &Node {
            unsafe {
                (*self.edges.get())[0]
            }
        }
    }

    // It would be nice if we could rely on lifetime elision and remove the `'a`
    // on the `foo` and `init` functions.
    fn foo(node: &'a Node) {
        println!("foo: {}", node.name);
    }

    fn init(arena: &'a Arena<Node>) -> &'a Node {
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
}

pub fn main() {
    let arena = Arena::new();
    // The lifetime of the module is inferred here from the lifetime of the
    // reference to the arena, i.e., the scope of the main function.
    let g = graph::init(&arena);
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    foo(g.first());
}
