use bevy::ecs::query::{WorldQuery, WorldQueryGats};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
pub const G: f32 = 1.0;

#[derive(Component, Clone)]
struct Lifeform;

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "bevytut".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_system(lifeform_apply_forces::<Without<Lifeform>, With<Lifeform>>)
        .add_system(lifeform_apply_forces::<With<Lifeform>, Without<Lifeform>>)
        .add_system(despawn_lifeform)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle { ..default() });
}

fn spawn_basic_scene(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 10.0,
        center: Vec2::ZERO,
    };

    let mut rng = rand::thread_rng();

    for _ in 0..2 {
        let x = rng.gen::<f32>() * WIDTH - (WIDTH / 4.0);
        let y = rng.gen::<f32>() * HEIGHT - (HEIGHT / 4.0);
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                },
            ))
            .insert(Lifeform);
    }
}
fn lifeform_apply_forces<F, F2>(
    mut lifeform: Query<&mut Transform, (With<Lifeform>, F)>,
    other_lifeform: Query<&Transform, (With<Lifeform>, F2)>,
) where
    F: WorldQuery,
    F2: WorldQuery,
    for<'a> <F as WorldQueryGats<'a>>::Fetch: Clone,
{
    let mut fx: f32 = 0.0;
    let mut fy: f32 = 0.0;

    for mut lifeform1 in lifeform.iter_mut() {
        for lifeform2 in other_lifeform.iter() {
            let dx = lifeform1.translation.x - lifeform2.translation.x;
            let dy = lifeform1.translation.y - lifeform2.translation.y;
            let d = (dx * dx + dy * dy).sqrt();
            if d > 0.0 {
                let f = G * 1.0 / d;
                fx += f * dx;
                fy += f * dy;
            }

            lifeform1.translation.x += fx;
            lifeform1.translation.y += fy;
        }
    }
}

fn despawn_lifeform(
    mut commands: Commands,
    mut shapes: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut shapes {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
