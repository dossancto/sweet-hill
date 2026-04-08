use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource)]
pub struct GunBag {
    pub guns: HashMap<String, (Entity)>,
    pub max_guns: usize,
}

impl GunBag {
    pub fn add_gun(&mut self, name: String, entity: Entity) {
        if self.guns.len() < self.max_guns {
            self.guns.insert(name, entity);
        }
    }

    /// Checks if there is an available slot for adding a new gun.
    ///
    /// Returns `true` if the current number of guns is less than the maximum allowed (`max_guns`),
    /// indicating that a new gun can be added. Returns `false` otherwise.
    pub fn guns_slot_available(&self) -> bool {
        self.guns.len() < self.max_guns
    }
}
