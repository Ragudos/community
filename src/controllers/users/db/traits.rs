pub trait Token {
    fn is_expired(&self) -> bool;
}
