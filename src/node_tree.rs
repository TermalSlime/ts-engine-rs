use std::collections::*;

pub struct Node {
    name: String,

    children: HashMap<String, Node>,
    components: Vec<Box<dyn Component>>
}

pub trait Component {

}

impl Node {
    pub fn create(name: String) -> Node {
        Node {
            name,
            children: HashMap::new(),
            components: Vec::new()
        }
    }

    pub fn add_child(&mut self, node: Node) {
        self.children.insert(node.name.clone(), node);
    }

    pub fn get_children(&self) -> Vec<&Node> {
        let mut children: Vec<Node> = Vec::new();
        let mut children_iter = self.children.values();
        for c in children_iter {
            children.push(c);
        }
        children
    }
}
