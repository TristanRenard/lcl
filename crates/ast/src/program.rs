use crate::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<Node>,
}