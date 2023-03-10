mod snake_game;

// use snake_game::SnakeGame;
use bevy::prelude::*;
// use snakeView::SnakeView;
pub static BLOCK_WIDTH: u32 = 64;
pub static BLOCK_HEIGHT: u32 = 48;
pub static BLOCK_SIZE: u32 = 10;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
// Controller
// struct Controller {
//     game: SnakeGame,
//     // view: SnakeView,
// }

#[derive(Component)]
struct SnakeHead;
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            // transform: Transform {
            //     scale: Vec3::new(10.0, 10.0, 10.0),
            //     ..default()
            // },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 }) // <--
        .insert(Size::square(0.8)); // <--;
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.0;
        }
    }
}

// pub struct View {
//     snakeGame: SnakeGame, // Rotation for the square.
// }

// impl View {
//     fn spawn_snake(mut commands: Commands) {}
// }

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, BLOCK_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, BLOCK_HEIGHT as f32),
            0.0,
        );
    }
}
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / BLOCK_WIDTH as f32 * window.width() as f32,
            sprite_size.height / BLOCK_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}
fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(position_translation).with_system(size_scaling)
        )
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::new_with_far(2000.0));
}