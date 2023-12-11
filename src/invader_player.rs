pub trait InvaderPlayer {
    pub fn shoot(&mut self) -> bool;
    pub fn update(&mut self, delta: Duration);
    pub fn detect_hits(&mut self, player: &mut Player);


}