use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct HomePlugin;
impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics);
    }
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 500.0;
    let ground_height = 10.0;

    commands.spawn((
        Transform::from_xyz(0.0, 0.0 * -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 50.0 * -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 100.0 * -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 150.0 * -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height),
    ));
}
