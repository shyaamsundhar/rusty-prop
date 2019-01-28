pub trait ModelTemplate {
    fn save(&self) -> u32;
    fn update(&self) -> bool;
    fn get(id:u32) -> Self;
}