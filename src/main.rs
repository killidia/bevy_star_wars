mod landscape;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    pbr::DirectionalLightShadowMap,
    prelude::*,
};
use landscape::{LandscapePlugin, MoveWithLandscapeTag};
use std::f32::consts::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_plugins(LandscapePlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                start_walker_animation,
                animate_light_direction,
                camera_input,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(WalkerAnimation(
        asset_server.load("models/walker/walker.gltf#Animation0"),
    ));

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.7, 20.0, 40.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        })
        .insert(CameraController {
            rotation: Quat::IDENTITY,
            zoom: 20.0,
        });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/xwing/xwing.gltf#Scene0"),
        ..default()
    });

    commands.spawn((
        MoveWithLandscapeTag,
        SceneBundle {
            scene: asset_server.load("models/walker/walker.gltf#Scene0"),
            transform: Transform::from_xyz(-30.0, -20.0, 0.0),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct CameraController {
    pub rotation: Quat,
    pub zoom: f32,
}

pub fn camera_input(
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut controller, mut transform) in query.iter_mut() {
        for wheel in mouse_wheel.iter() {
            controller.zoom -= wheel.y;
        }

        if buttons.pressed(MouseButton::Right) {
            for mouse in mouse_motion.iter() {
                let delta = mouse.delta * time.delta_seconds() * 0.3;
                controller.rotation *= Quat::from_euler(EulerRot::XYZ, delta.x, delta.y, 0.0);
            }
        }
        transform.translation = controller.rotation * Vec3::Z * controller.zoom;
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

#[derive(Resource)]
pub struct WalkerAnimation(pub Handle<AnimationClip>);

fn start_walker_animation(
    walker_animation: Res<WalkerAnimation>,
    mut walkers: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut animation_player in walkers.iter_mut() {
        animation_player
            .play(walker_animation.0.clone_weak())
            .repeat();
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
