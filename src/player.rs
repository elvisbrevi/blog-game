use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Componente que define las propiedades del jugador
#[derive(Component)]
pub struct Player {
    /// Player speed factor.
    pub speed: f32,
}

/// Sistema que configura el jugador
pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = Sprite::from_image(asset_server.load("test.png"));
    let size = Vec2::new(50.0, 50.0);

    commands
        .spawn((
            Player { speed: 200.0 },
            sprite,
            Transform::from_xyz(size.x, size.y, 0.),
            RigidBody::Dynamic,
            Collider::cuboid(size.x, size.y),
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Dominance::group(10));
}

/// Sistema que maneja el movimiento del jugador
pub fn move_player(
    query: Single<(&Player, &mut Transform)>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let (player, mut transform) = query.into_inner();
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * player.speed * time.delta_secs();
    transform.translation.x += move_delta.x;
    transform.translation.y += move_delta.y;
}

/// Plugin que encapsula toda la funcionalidad del jugador
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, move_player)
            .insert_resource(ClearColor(Color::srgb(
                0xF9 as f32 / 255.0,
                0xF9 as f32 / 255.0,
                0xFF as f32 / 255.0,
            )))
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Startup, setup_physics);
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

    /*
     * Create the cubes
     */
    let num = 8;
    let rad = 10.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            let x = i as f32 * shift - centerx + offset;
            let y = j as f32 * shift + centery + 30.0;

            commands.spawn((
                Transform::from_xyz(x, y, 0.0),
                RigidBody::Dynamic,
                Collider::cuboid(rad, rad),
            ));
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}
