use bevy::prelude::*;

/// Represents the ammunition state for a gun, including clip and stock counts.
///
/// This component can be attached to an entity to track its ammo usage, reloading, and depletion.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmo {
    /// The maximum number of bullets that can be loaded into the gun's clip at once.
    pub max_clip_size: u16,

    /// The maximum number of bullets that can be held in reserve (stock).
    pub max_stock_size: u16,

    /// The current number of bullets loaded in the gun's clip (ready to fire).
    pub current_on_clip: u16,

    /// The current number of bullets available in reserve (stock), not yet loaded into the clip.
    pub current_on_stock: u16,
}

impl GunAmmo {
    /// Returns `true` if the clip is fully loaded (i.e., `current_on_clip` equals `max_clip_size`).
    pub fn full_clip(&self) -> bool {
        self.current_on_clip == self.max_clip_size
    }

    /// Returns `true` if there is at least one bullet in the clip (`current_on_clip` > 0).
    ///
    pub fn has_ammo(&self) -> bool {
        self.current_on_clip > 0
    }

    /// Returns `true` if there are no bullets left in the stock (`current_on_stock` == 0).
    pub fn empty_stock(&self) -> bool {
        self.current_on_stock == 0
    }

    /// Reloads the clip from the stock, transferring as many bullets as needed (and available) to fill the clip.
    ///
    /// - If the clip is already full or the stock is empty, nothing happens.
    /// - Otherwise, moves bullets from `current_on_stock` to `current_on_clip` up to `max_clip_size`.
    pub fn reload(&mut self) {
        let needed_ammo = self.max_clip_size - self.current_on_clip;
        let ammo_to_reload = needed_ammo.min(self.current_on_stock);
        self.current_on_clip += ammo_to_reload;
        self.current_on_stock -= ammo_to_reload;
    }

    /// Decreases the number of bullets in the clip by one, if there is ammo available.
    ///
    /// - If the clip is empty, nothing happens.
    /// - The value will not go below zero.
    pub fn decrease_ammo(&mut self) {
        if self.has_ammo() == false {
            return;
        }
        self.current_on_clip -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_clip() {
        let ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 10,
            current_on_stock: 20,
        };
        assert!(ammo.full_clip());
        let ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 5,
            current_on_stock: 20,
        };
        assert!(!ammo.full_clip());
    }

    #[test]
    fn test_has_ammo() {
        let ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 1,
            current_on_stock: 20,
        };
        assert!(ammo.has_ammo());
        let ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 0,
            current_on_stock: 20,
        };
        assert!(!ammo.has_ammo());
    }

    #[test]
    fn test_decrease_ammo() {
        let mut ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 2,
            current_on_stock: 20,
        };
        ammo.decrease_ammo();
        assert_eq!(ammo.current_on_clip, 1);
        ammo.decrease_ammo();
        assert_eq!(ammo.current_on_clip, 0);
        // Should not go below zero
        ammo.decrease_ammo();
        assert_eq!(ammo.current_on_clip, 0);
    }

    #[test]
    fn test_reload_partial() {
        // Reload when there is enough stock to fully reload
        let mut ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 3,
            current_on_stock: 20,
        };
        ammo.reload();
        assert_eq!(ammo.current_on_clip, 10);
        assert_eq!(ammo.current_on_stock, 13);
    }

    #[test]
    fn test_reload_insufficient_stock() {
        // Reload when there is not enough stock to fully reload
        let mut ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 5,
            current_on_stock: 3,
        };
        ammo.reload();
        assert_eq!(ammo.current_on_clip, 8);
        assert_eq!(ammo.current_on_stock, 0);
    }

    #[test]
    fn test_reload_full_clip() {
        // Reload when the clip is already full
        let mut ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 10,
            current_on_stock: 20,
        };
        ammo.reload();
        assert_eq!(ammo.current_on_clip, 10);
        assert_eq!(ammo.current_on_stock, 20);
    }

    #[test]
    fn test_reload_empty_stock() {
        // Reload when there is no stock left
        let mut ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 2,
            current_on_stock: 0,
        };
        ammo.reload();
        assert_eq!(ammo.current_on_clip, 2);
        assert_eq!(ammo.current_on_stock, 0);
    }
}
