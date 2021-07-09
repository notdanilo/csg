mod position;
pub use position::Position;

#[derive(Default)]
pub struct Field3<Type: Default> {
    data: Vec<Type>,
    dimensions: usize
}

impl<Type: Default> Field3<Type> {
    pub fn new() -> Self {
        Default::default()
    }
}