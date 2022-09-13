use super::deep::DeepNode;
use super::AtomNode;
use crate::parser::ParserError;

const COMMENT_PREFIX: &str = "#";

pub enum PrefixNode {
    Atom(AtomNode),
    Symbol(String),
    List(Vec<Vec<PrefixNode>>),
    Map(Vec<(Vec<PrefixNode>, Vec<PrefixNode>)>),
    Infix(Vec<PrefixNode>),
    Top(Vec<PrefixNode>),
}

pub fn parse(deep_nodes: Vec<DeepNode>) -> Result<Vec<PrefixNode>, ParserError> {
    let mut iter = deep_nodes.into_iter();
    let mut prefix_nodes = Vec::new();
    while let Some(deep_node) = iter.next() {
        let prefix_node = match deep_node {
            DeepNode::Atom(a) => PrefixNode::Atom(a),
            DeepNode::Symbol(s) => match s.as_str() {
                COMMENT_PREFIX => {
                    if iter.next().is_none() {
                        return ParserError::err("expect comment body".to_owned());
                    } else {
                        continue;
                    }
                }
                _ => PrefixNode::Symbol(s),
            },
            DeepNode::List(l) => {
                let mut list = Vec::with_capacity(l.len());
                for node in l {
                    let item = parse(node)?;
                    // drop comments
                    if !item.is_empty() {
                        list.push(item);
                    }
                }
                PrefixNode::List(list)
            }
            DeepNode::Map(m) => {
                let mut map = Vec::with_capacity(m.len());
                for pair in m {
                    let key = parse(pair.0)?;
                    let value = parse(pair.1)?;
                    // drop comments
                    if !key.is_empty() && !value.is_empty() {
                        map.push((key, value));
                    }
                }
                PrefixNode::Map(map)
            }
            DeepNode::Infix(i) => {
                let nodes = parse(i)?;
                // drop comments
                if nodes.is_empty() {
                    continue;
                } else {
                    PrefixNode::Infix(nodes)
                }
            }
            DeepNode::Top(t) => {
                let nodes = parse(t)?;
                // drop comments
                if nodes.is_empty() {
                    continue;
                } else {
                    PrefixNode::Top(nodes)
                }
            }
        };
        prefix_nodes.push(prefix_node);
    }
    Ok(prefix_nodes)
}