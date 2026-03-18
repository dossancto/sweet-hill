use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GunAmmo {
    pub magazine_size: u16,
    pub stock_size: u16,
    pub current_ammo: u16,
    pub current_stock_ammo: u16,
}

impl GunAmmo {
    pub fn full_clip(&self) -> bool {
        self.current_ammo == self.magazine_size
    }

    pub fn has_ammo(&self) -> bool {
        self.current_ammo > 0
    }

    pub fn reload(&mut self) {
        self.current_ammo = self.magazine_size;
        self.current_stock_ammo = self.current_stock_ammo;
    }

    pub fn decrease_ammo(&mut self) {
        if self.has_ammo() == false {
            return;
        }
        self.current_ammo -= 1;
    }
}
