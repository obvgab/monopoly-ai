use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Money { pub worth: i32 }
#[derive(Component, Inspectable)]
pub struct Token { pub current: i32, pub previous: i32 }
#[derive(Component, Inspectable)]
pub struct JailFree { pub held: i32 }

#[derive(Component, Inspectable)]
#[component(storage="SparseSet")]
pub struct Computer;
#[derive(Component, Inspectable)]
#[component(storage="SparseSet")]
pub struct Jailed;

/*
    The actual player bundle that contains
    the above components. This allows for
    easy insertions via .insert_bundle()
*/
#[derive(Bundle)]
pub struct PlayerBundle {
    pub money: Money,
    pub token_position: Token,
    pub held_jail_free: JailFree
}
