use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, SamplerDescriptor, Texture, TextureDescriptor, TextureDimension, TextureFormat,
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
        // .add_system(update_screen_texture)
        .run();
}

// https://bevy-cheatbook.github.io/programming/non-send.html
fn setup_exclusive(world: &mut World) {
    // Load the screen capturer as a resource
    let display = Display::primary().unwrap();
    let capturer = Capturer::new(display).unwrap();
    world.insert_non_send_resource(capturer);
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    // Create a camera and a 2D sprite to display the screen texture
    commands.spawn(Camera2dBundle::default());
    // let sprite_handle = materials.add(ColorMaterial::color(Color::rgba(1.0, 1.0, 1.0, 0.0)));

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
    // for (x, y, pixel) in image. {
    //     let offset = (y * width + x) as usize * 4;
    //     let r = frame[offset + 0];
    //     let g = frame[offset + 1];
    //     let b = frame[offset + 2];
    //     let a = frame[offset + 3];
    //     *pixel = Image::Rgba([r, g, b, a]);
    // }

    // image.save("screenshot.png").unwrap();
}

// struct CaptureHelpers(Box<Capturer>);

// fn update_screen_texture(
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     capture_helpers: NonSend<CaptureHelpers>,
//     mut images: ResMut<Assets<Image>>,
//     render_resource_context: Res<Box<dyn RenderResourceContext>>,
// ) {
//     let capturer = &mut *capture_helpers.0;

//     // Capture the screen and update the texture
//     let Ok(frame) = capturer.frame() else { return };
//     let width = capturer.width() as u32;
//     let height = capturer.height() as u32;
//     // let texture = Image::new(
//     //     Extent3d { width, height, depth_or_array_layers: 1 },
//     //     TextureDimension::D2,
//     //     frame.to_vec(),
//     //     TextureFormat::Rgba8UnormSrgb,
//     // );

//     // // Apply the texture to the 2D sprite
//     // if let Some(material) = materials.iter_mut().next() {
//     //     material.1.texture = Some(texture.into());
//     // }

//     let texture = render_resource_context.create_texture(bevy::render::texture::Texture {
//         size: Extent3d {
//             width,
//             height,
//             depth_or_array_layers: 1,
//         },
//         dimension: TextureDimension::D2,
//         format: TextureFormat::Bgra8UnormSrgb,
//         mip_level_count: 1,
//         sample_count: 1,
//         sampler: SamplerDescriptor::default(),
//     });

//     render_resource_context.write_texture(
//         texture.create_default_view(),
//         0,
//         0,
//         bevy::render::texture::TextureDataLayout {
//             offset: 0,
//             bytes_per_row: std::num::NonZeroU32::new(width * 4),
//             rows_per_image: std::num::NonZeroU32::new(height),
//         },
//         Extent3d {
//             width,
//             height,
//             depth_or_array_layers: 1,
//         },
//         std::borrow::Cow::Borrowed(frame.as_ref()),
//     );

//     let image = Image {
//         texture,
//         ..Default::default()
//     };
//     let image_handle = images.add(image);

//     if let Some(material) = materials.iter_mut().next() {
//         material.1.texture = Some(image_handle);
//     }
// }
