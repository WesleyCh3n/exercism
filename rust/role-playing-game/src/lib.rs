// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        /* match self.health.cmp(&0) {
         *     std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
         *         Some(Self {
         *             health: 100,
         *             mana: {
         *                 if self.level >= 10 {
         *                     Some(100)
         *                 } else {
         *                     None
         *                 }
         *             },
         *             ..*self
         *         })
         *     }
         *     std::cmp::Ordering::Greater => None,
         * } */
        (self.health == 0).then(|| Self{
            health: 100,
            mana: self.level.ge(&10).then(|| 100),
            ..*self
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana_cost > mana => 0,
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                2 * mana_cost
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                /* self.health = if self.health >= mana_cost  {
                 *     self.health - mana_cost
                 * } else {
                 *     0
                 * }; */
                0
            }
        }
    }
}
