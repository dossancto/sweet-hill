use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmo {
    pub max_clip_size: u16,
    pub max_stock_size: u16,
    pub current_on_clip: u16,
    pub current_on_stock: u16,
}

impl GunAmmo {
    pub fn full_clip(&self) -> bool {
        self.current_on_clip == self.max_clip_size
    }

    pub fn has_ammo(&self) -> bool {
        self.current_on_clip > 0
    }

    pub fn reload(&mut self) {
        let needed_ammo = self.max_clip_size - self.current_on_clip;
        let ammo_to_reload = needed_ammo.min(self.current_on_stock);
        self.current_on_clip += ammo_to_reload;
        self.current_on_stock -= ammo_to_reload;
    }

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
    fn test_reload() {
        let mut ammo = GunAmmo {
            max_clip_size: 10,
            max_stock_size: 30,
            current_on_clip: 2,
            current_on_stock: 20,
        };
        ammo.reload();
        assert_eq!(ammo.current_on_clip, 10);
        // current_stock_ammo should remain unchanged
        assert_eq!(ammo.current_on_stock, 20);
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
}
