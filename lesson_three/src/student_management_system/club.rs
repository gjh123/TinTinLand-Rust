#[derive(Debug)]
pub struct Club {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) members: Vec<u32>,
}
