pub trait Area {
    type ItemType;
    fn at(&self, x: usize, y: usize) -> Self::ItemType;
}
