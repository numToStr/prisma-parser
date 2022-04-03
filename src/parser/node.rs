use super::{datasource::Datasource, generator::Generator, r#enum::Enum};

#[derive(Debug)]
pub enum Node {
    Datasource(Datasource),
    Generator(Generator),
    Enum(Enum),
}

#[derive(Debug, Default)]
pub struct Document {
    nodes: Vec<Node>,
}

impl Document {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node)
    }
}
