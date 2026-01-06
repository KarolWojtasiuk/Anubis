use crate::prelude::*;

#[derive(Default)]
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            update_camera_position.in_set(GameplayPostUpdateSystems::CameraUpdate),
        );
        app.add_systems(
            PostUpdate,
            (update_camera_target_zoom, update_camera_zoom)
                .chain()
                .in_set(GameplayPostUpdateSystems::CameraUpdate),
        );
    }
}

#[derive(Component, SmartDefault)]
#[require(Transform, Camera3d)]
pub struct GameCamera {
    #[default(45)]
    pub target_zoom: u8,
    #[default(15)]
    pub min_zoom: u8,
    #[default(150)]
    pub max_zoom: u8,
}

#[derive(Component)]
#[require(Transform)]
pub struct GameCameraTarget {
    pub offset: Vec3,
}

fn update_camera_position(
    mut camera: Single<&mut Transform, (With<GameCamera>, Without<GameCameraTarget>)>,
    target: Single<(&Transform, &GameCameraTarget), Without<GameCamera>>,
    time: Res<Time>,
) {
    camera.translation.smooth_nudge(
        &(target.0.translation + target.1.offset),
        10.0,
        time.delta_secs(),
    );
    camera.look_to(-target.1.offset, Dir3::Y);
}

fn update_camera_target_zoom(mut camera: Single<&mut GameCamera>, input: Res<GameplayInput>) {
    if input.zoom != 0 {
        let target_zoom = {
            let mut zoom = camera.target_zoom as i16;
            zoom -= input.zoom as i16 * 5;
            zoom.clamp(u8::MIN as i16, u8::MAX as i16) as u8
        };
        camera.target_zoom = target_zoom.clamp(camera.min_zoom, camera.max_zoom);
    }
}

fn update_camera_zoom(
    mut camera: Single<(&GameCamera, &mut Projection)>,
    time: Res<Time>,
) -> Result {
    let (camera, ref mut projection) = *camera;
    let Projection::Perspective(ref mut perspective) = **projection else {
        return Err("Cannot update camera zoom when not using perspective projection".into());
    };

    let target_fov = (camera.target_zoom as f32).to_radians();
    perspective
        .fov
        .smooth_nudge(&target_fov, 5.0, time.delta_secs());

    Ok(())
}
