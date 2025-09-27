use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

fn main() {
   App::new()
      .add_plugins(DefaultPlugins)
      .add_systems(Startup, (spawn_track, spawn_player))
      .add_systems(Update, player_move)
      .run();
}

#[derive(Component)]
struct Track {
   radius: f32,
   length: f32
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Speed(f32);

fn spawn_track(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
   //let shape = meshes.add(Annulus::new(400.0, 600.0));
   let shape = meshes.add(Capsule2d::new(225.0, 500.0));
   let color = Color::hsl(35.0, 0.69, 0.63);
   commands.spawn((
      Mesh2d(shape),
      MeshMaterial2d(materials.add(color)),
      Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
      Track {radius: 225.0, length: 500.0 }
   ));

   let shape = meshes.add(Capsule2d::new(112.5, 500.0));
   let color = Color::hsl(99.0, 0.66, 0.55);
   commands.spawn((
      Mesh2d(shape),
      MeshMaterial2d(materials.add(color)),
      Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2))
   ));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
   commands.spawn(Camera2d);
   commands.spawn((
      Name::new("Player"),
      Sprite::from_image(asset_server.load("icon.png")),
      Transform::from_xyz(0.0, 160.0, 0.0)
         .with_scale(Vec3::splat(0.2)), // sprite scale
      Speed(20.0),
      Velocity(Vec2::new(-20.0, 0.0))
   ));
}

fn player_move() {
   
}

/* 
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(), */