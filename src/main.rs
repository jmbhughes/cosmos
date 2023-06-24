mod uw;

use std::cell;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PIXELS_PER_SIDE: u8 = 5;
const ACTIVE_COLOR: Color = Color::rgb(0.25, 0.25, 0.75);
const DEAD_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const NUM_ROWS: usize = 100;
const NUM_COLS: usize = 100;


fn main() {
    let mut ca = uw::UlamWarburtonCA::new(NUM_ROWS, NUM_COLS);
    ca.set(49, 49);
    ca.set(80, 80);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ca)
        .add_system(visualize)
        .add_system(step_ca)
        .run();
}

fn step_ca(mut ca: ResMut<uw::UlamWarburtonCA>
) {
    ca.step();
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
    mut ca: ResMut<uw::UlamWarburtonCA>,
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
                    i as f32 * PIXELS_PER_SIDE as f32 - (NUM_ROWS) as f32, 
                    j as f32 * PIXELS_PER_SIDE as f32 - (NUM_COLS) as f32,
                    0.)),
                ..default()
            }));
        }
    }
}

