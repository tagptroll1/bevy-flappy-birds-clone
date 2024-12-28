// use bevy::{
//     dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
//     prelude::*,
//     text::FontSmoothing,
// };
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
//
// #[derive(Component)]
// pub struct Debuggable;
// #[derive(Component)]
// struct Hitbox;
//
// pub struct DebugPlugin;
//
// struct OverlayColor;
// impl OverlayColor {
//     const GREEN: Color = Color::srgb(0., 1., 0.);
// }
//
// impl Plugin for DebugPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(FpsOverlayPlugin {
//             config: FpsOverlayConfig {
//                 text_config: TextFont {
//                     font_size: 22.0,
//                     font: default(),
//                     font_smoothing: FontSmoothing::default(),
//                 },
//                 text_color: OverlayColor::GREEN,
//                 enabled: true,
//             }
//         })
//             .add_plugins(WorldInspectorPlugin::new())
//             .add_systems(Update, print_transform)
//             .add_systems(Update, customize_config);
//     }
// }
// fn print_transform(query: Query<(Entity, &Transform)>) {
//     // for (entity, transform) in query.iter() {
//     //     //info!("Entity: {:?}, Transform: {:?}", entity, transform.translation);
//     // }
// }
//
// fn customize_config(
//     mut commands: Commands,
//     input: Res<ButtonInput<KeyCode>>,
//     mut overlay: ResMut<FpsOverlayConfig>,
//     debug_q: Query<(Entity, &Transform), With<Debuggable>>,
//     hitbox_q: Query<Entity, With<Hitbox>>,
// ) {
//     if input.just_pressed(KeyCode::Digit1) {
//         overlay.enabled = !overlay.enabled;
//     }
//     if input.just_pressed(KeyCode::Digit2) {
//         for (entity, transform) in debug_q.iter() {
//             let has_hitbox = hitbox_q.get(entity).is_ok();
//
//             if has_hitbox {
//                 if let Ok(child) = hitbox_q.get(entity) {
//                     commands.entity(entity).despawn();
//                 }
//             } else {
//                 commands.entity(entity).with_child((
//                     Sprite {
//                         color: Color::srgba(1.0, 0.0, 0.0, 0.5),
//                         custom_size: Some(Vec2::new(transform.scale.x, transform.scale.y)),
//                         ..default()
//                     },
//                     Hitbox
//                 ));
//             }
//         }
//     }
// }
