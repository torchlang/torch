#[derive(Debug)]
pub enum Table {
    Id(Option<String>),
    Illegal(Option<String>),
}
impl Table {
    #[inline]
    pub async fn default() -> Self {
        Self::Illegal(None)
    }
}
