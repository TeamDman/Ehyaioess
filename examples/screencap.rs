use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages,
};

use scrap::{Capturer, Display};
// https://github.com/quadrupleslap/scrap

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_exclusive)
        .add_system(bevy::window::close_on_esc)
        .add_system(screenshot_trigger_system)
        .run();
}

// https://bevy-cheatbook.github.io/programming/non-send.html
fn setup_exclusive(world: &mut World) {
    let display = Display::primary().unwrap();
    let capturer = Capturer::new(display).unwrap();
    world.insert_non_send_resource(capturer);
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 1920,
        height: 1080,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: image_handle.clone(),
        ..Default::default()
    });

    commands.spawn(MainThing(image_handle));
}

#[derive(Component)]
struct MainThing(Handle<Image>);

fn screenshot_trigger_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut capturer: NonSendMut<Capturer>,
    mut query: Query<&mut MainThing>,
    mut images: ResMut<Assets<Image>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
    let width = capturer.width() as u32;
    let height = capturer.height() as u32;
    let Ok(frame) = capturer.frame() else { return };
    let thing = query.single_mut();
    let Some(image) = images.get_mut(&thing.0) else { return };
    image.resize(Extent3d {
        width,
        height,
        ..default()
    });
    image.data = frame.to_vec();
}