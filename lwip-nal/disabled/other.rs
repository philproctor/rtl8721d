
#[derive(Debug)]
pub enum ChainBuffer {
    Empty,
    Buffer(Vec<u8>),
    Pbuf(Option<Pbuf>),
    Link(Vec<u8>, Box<ChainBuffer>),
}

impl ChainBuffer {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn add_link(&mut self, buf: Pbuf) {
        self.clear_pbuf();
        match self {
            Self::Buffer(b) => {
                *self = Self::Link(replace(b, Vec::new()), Box::new(Self::Pbuf(Some(buf))))
            }
            Self::Empty => *self = Self::Pbuf(Some(buf)),
            Self::Link(_, next) => next.add_link(buf),
            _ => panic!("Unexpected!"),
        }
    }

    fn clear_pbuf(&mut self) {
        match self {
            Self::Pbuf(buf) => *self = Self::Buffer(replace(buf, None).unwrap().consume()),
            _ => {}
        };
    }

    pub fn pull_if_needed(&mut self) {
        self.clear_pbuf();
        match self {
            Self::Link(buf, next) => {
                if buf.len() == 0 {
                    *self = replace(next, ChainBuffer::Empty);
                }
            }
            _ => {}
        };
    }

    pub fn read_segment(&mut self) -> Option<&mut Vec<u8>> {
        self.clear_pbuf();
        match self {
            Self::Empty => None,
            Self::Buffer(b) | Self::Link(b, _) => Some(b),
            _ => panic!("Unexpected!"),
        }
    }
}