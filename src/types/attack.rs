#[derive(Debug, Default)]
pub struct Attack {
    weapon_name: String,
    damage: Option<u32>,
}

impl Attack {
    pub fn new(weapon_name: String, damage: Option<u32>) -> Self {
        Self {
            weapon_name,
            damage,
        }
    }

    pub fn damage(&self) -> Option<u32> {
        self.damage
    }

    pub fn weapon_name(&self) -> &String {
        &self.weapon_name
    }
}
