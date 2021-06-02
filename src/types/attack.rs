#[derive(Debug, Default)]
pub struct Attack {
    weapon_name: String,
    damage: Option<u32>,
}

impl Attack {
    pub fn new<S>(weapon_name: S, damage: Option<u32>) -> Self
    where
        S: Into<String>,
    {
        Self {
            weapon_name: weapon_name.into(),
            damage,
        }
    }

    pub fn damage(&self) -> Option<u32> {
        self.damage
    }

    pub fn weapon_name(&self) -> &str {
        &self.weapon_name
    }
}
