use crate::commands::Line;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
    None,
    Link { item: Line, next: Box<Node> },
}

impl Node {
    pub fn push(&mut self, val: Line) {
        *self = match self {
            Self::Link { item, next } => {
                next.push(val);
                Self::Link {
                    item: item.clone(),
                    next: next.clone(),
                }
            }
            Self::None => Self::Link {
                item: val,
                next: Box::new(Self::None),
            },
        }
    }

    pub fn find_line(&self, line: usize) -> Option<Node> {
        if let Self::Link { item, next } = self {
            if item.0 == line {
                Some(self.clone())
            } else {
                next.find_line(line)
            }
        } else {
            None
        }
    }
}
