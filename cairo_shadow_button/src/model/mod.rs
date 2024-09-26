#[derive(Default, Debug, Clone)]
pub struct Model {
    value: u8,
}
impl Model {
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn inc(&mut self) {
        if self.value < 255 {
            self.value = self.value.saturating_add(1);
        }
    }
    pub fn dec(&mut self) {
        if self.value > 0 {
            self.value = self.value.saturating_sub(1);
        }
    }
}
