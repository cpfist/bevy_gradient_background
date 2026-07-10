use bevy::{
    camera::visibility::RenderLayers,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

#[derive(Debug)]
pub struct GradientBackgroundPlugin {
    /// Gradient top color
    pub top_color: Color,
    /// Gradient bottom color
    pub bottom_color: Color,
    /// Render layer where gradient is stored (default: 1)
    pub render_layer: usize,
}

impl Default for GradientBackgroundPlugin {
    fn default() -> Self {
        Self {
            top_color: Color::srgb_from_array([0.035, 0.035, 0.045]),
            bottom_color: Color::srgb_from_array([0.12, 0.12, 0.15]),
            render_layer: 1,
        }
    }
}

impl Plugin for GradientBackgroundPlugin {
    fn build(&self, app: &mut App) {
        let t = self.top_color.to_srgba();
        let b = self.bottom_color.to_srgba();

        app.add_plugins(Material2dPlugin::<GradientMaterial>::default())
            .insert_resource(GradientColors {
                top: Vec4::new(t.red, t.green, t.blue, 1.0),
                bottom: Vec4::new(b.red, b.green, b.blue, 1.0),
            })
            .insert_resource(Layer(self.render_layer))
            .add_systems(Startup, setup)
            .add_systems(Update, resize_background);
    }
}

#[derive(Debug, Resource)]
struct Layer(usize);

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct GradientMaterial {
    #[uniform(0)]
    colors: GradientColors,
}

#[derive(ShaderType, Debug, Clone, Resource)]
struct GradientColors {
    top: Vec4,
    bottom: Vec4,
}

impl Material2d for GradientMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/gradient.wgsl".into()
    }
}

#[derive(Component)]
struct Background;

fn resize_background(windows: Query<&Window>, mut query: Query<&mut Transform, With<Background>>) {
    let Ok(window) = windows.single() else {
        return;
    };

    let size = Vec2::new(window.width(), window.height());

    for mut transform in &mut query {
        transform.scale = Vec3::new(size.x / 2.0, size.y / 2.0, 1.0);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GradientMaterial>>,
    gradient_colors: Res<GradientColors>,
    layer: Res<Layer>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            ..default()
        },
        RenderLayers::layer(layer.0),
    ));

    let mesh = Rectangle::new(2.0, 2.0);
    let mesh = meshes.add(mesh);

    let background_material = materials.add(GradientMaterial {
        colors: gradient_colors.clone(),
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(background_material),
        Background,
        RenderLayers::layer(layer.0),
    ));
}
