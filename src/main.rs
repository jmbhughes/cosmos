mod uw;

use std::cmp;
use std::cell;
use std::time::Duration;

use bevy::{prelude::*, 
    sprite::MaterialMesh2dBundle,
    window::{CursorGrabMode, PresentMode, WindowLevel}};

const PIXELS_PER_SIDE: usize = 5;
const ACTIVE_COLOR: Color = Color::rgb(0.25, 0.25, 0.75);
const DEAD_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const NUM_ROWS: usize = 200;
const NUM_COLS: usize = 200;
const SPEED_MILLIS: u64 = 100;


fn main() {
    let mut ca = uw::AdvancedUlamWarburtonCA::new(NUM_ROWS, NUM_COLS, 5, 1);
    ca.set(99, 99);
    ca.set(100, 99);
    ca.set(98, 99);
    ca.set(99, 100);
    ca.set(99, 98);

    let window_size: f32 = (cmp::max(NUM_ROWS, NUM_COLS) * PIXELS_PER_SIDE) as f32;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cosmos".into(),
                resolution: (window_size, window_size).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .insert_resource(ca)
        .insert_resource(UpdateTimer { timer: Timer::new(Duration::from_millis(SPEED_MILLIS), TimerMode::Repeating)})
        .add_system(visualize)
        .add_system(step_ca)
        .run();
}

#[derive(Resource)]
struct UpdateTimer {
    timer: Timer,
}

fn step_ca(mut ca: ResMut<uw::AdvancedUlamWarburtonCA>, 
    mut update_timer: ResMut<UpdateTimer>,
    time: Res<Time>,
) {
    // tick the timer
    update_timer.timer.tick(time.delta());

    if update_timer.timer.finished() {
        ca.step();
    }
}

#[derive(Component)]
pub struct Cell {
    pub i: usize,
    pub j: usize
}

fn visualize(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ca: ResMut<uw::AdvancedUlamWarburtonCA>,
    mut cell_query: Query<(&Cell, &mut Sprite,  &Transform)>,
) {
    for (cell, mut sprite, transform) in cell_query.iter_mut() {
        let i: usize = cell.i;
        let j: usize = cell.j;
        let color = match ca.get(i, j) {
            1 => ACTIVE_COLOR,
            _ => DEAD_COLOR
        };
        sprite.color.set_r(color.r());
        sprite.color.set_g(color.g());
        sprite.color.set_b(color.b());
    }
}


fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..NUM_ROWS {
        for j in 0..NUM_COLS {
            commands.spawn((
                Cell{i: i, j: j},
                SpriteBundle {
                sprite: Sprite {
                    color: DEAD_COLOR,
                    custom_size: Some(Vec2::new(PIXELS_PER_SIDE as f32, PIXELS_PER_SIDE as f32)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    i as f32 * PIXELS_PER_SIDE as f32 - ((NUM_ROWS * PIXELS_PER_SIDE) as f32 / 2.0), 
                    j as f32 * PIXELS_PER_SIDE as f32 - ((NUM_COLS * PIXELS_PER_SIDE) as f32 / 2.0),
                    0.)),
                ..default()
            }));
        }
    }
}

