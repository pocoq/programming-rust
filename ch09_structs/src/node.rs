use std::rc::Rc;

struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }

    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

pub fn handle_node() {
    let mut root = Node::new("html");
    let head = Node::new("head");
    let body = Node::new("body");

    Rc::new(head).append_to(&mut root);
    Rc::new(body).append_to(&mut root);

    println!("Root node: {:?}", root.tag);
    for child in &root.children {
        println!("Child node: {:?}", child.tag);
    }
}
