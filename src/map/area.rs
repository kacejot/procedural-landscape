pub trait Area
where
    Self::ItemType: Copy,
{
    type ItemType;
    fn at(&self, x: usize, y: usize) -> Self::ItemType;
}
