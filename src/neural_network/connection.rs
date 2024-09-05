use std::cmp::Ordering;

#[derive(Debug, Clone, PartialOrd)]
pub struct Connection {
    pub source: u16,
    pub target: u16,
    pub weight: f32,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.target == other.target && self.weight == other.weight
    }
}

impl Eq for Connection {}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source
            .cmp(&other.source)
            .then(self.target.cmp(&other.target))
            .then(self.weight.partial_cmp(&other.weight).unwrap())
    }
}
