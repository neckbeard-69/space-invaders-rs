pub trait Damageable {
    fn take_damage(&mut self, damage_amount: i16);
    fn is_alive(&self) -> bool;
}
