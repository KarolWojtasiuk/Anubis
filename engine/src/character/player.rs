use crate::prelude::*;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.in_set(GameplayUpdateSystems::General));
    }
}

#[derive(Component)]
#[require(Character, Name::from("Player"))]
pub struct Player;

fn move_player(
    mut player: Single<(&mut Transform, &Speed), With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let (ref mut transform, speed) = *player;
    let direction = Vec3::new(input.movement.x, 0.0, -input.movement.y);
    let speed = speed.0 as f32 * if input.sprint { 2.0 } else { 1.0 };
    transform.translation += direction * time.delta_secs() * speed;
}
