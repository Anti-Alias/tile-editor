
use std::f32::consts::PI;
use cgmath::{InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use wgpu::{Device, Queue, TextureFormat};
use tile_editor::app::{App, AppEvent};
use tile_editor::graphics::{Camera, Color, MaterialBuilder, Mesh, Model, ModelInstance, ModelInstanceSet, Texture};
use tile_editor::graphics::light::{AmbientLight, LightBundle, PointLight};
use tile_editor::graphics::scene::Scene;
use tile_editor::graphics::util::Matrix4Ext;

fn main() {
    let mut t = 0.0;
    let app = App::new()
        .title("Tile Editor")
        .size(1280, 720)
        .gui_enabled(false)
        .event_handler(move |event, state, _control_flow| {
            match event {
                AppEvent::STARTED => {
                    on_start(state.scene, state.device, state.queue)
                }
                AppEvent::UPDATE { dt } => {
                    t += dt;
                    on_update(state.scene, t)
                }
                AppEvent::RESIZED { width, height } => {
                    on_resize(width, height, state.scene)
                }
            }
        });
    app.start();
}

fn on_start(scene: &mut Scene, device: &Device, queue: &Queue) {
    log::info!("Application started!");

    let model_instances = create_box_model_and_instances(&device, &queue);
    let floor_instance = create_floor_and_instances(&device, &queue);
    scene.add_model_and_instances(&device, model_instances);
    scene.add_model_and_instances(&device, floor_instance);
    add_lights(scene);
}

fn on_update(scene: &mut Scene, t: f32) {
    move_camera(&mut scene.camera(), 50.0, t, 300.0);
    move_lights(&mut scene.light_bundle(), 200.0, t*1.414);
}

fn on_resize(width: u32, height: u32, _scene: &mut Scene) {
    log::info!("Application resized to dimensions: {}x{}", width, height)
}


fn move_lights(light_bundle: &mut LightBundle, radius: f32, t: f32) {
    let t = t/2.0;
    let point_lights = &mut light_bundle.point_lights;
    for (i, light) in point_lights.lights.iter_mut().enumerate() {
        let theta = PI * t / (i+1) as f32;
        let light_pos = &mut light.position;
        light_pos[0] = f32::cos(theta / 2.0) * radius;
        light_pos[2] = f32::sin(theta / 2.0) * radius;
    }
}

fn move_camera(camera: &mut Camera, y: f32, t: f32, rad: f32) {
    let t = t/2.0;
    let th = t * PI / 2.0;
    camera.move_to(Point3::new(
        f32::cos(th)*rad,
        f32::sin(th)*rad/2.0 + y,
        f32::sin(th)*rad)
    );
    camera.look_at(Point3::new(0.0, 0.0, 0.0));
}

fn add_lights(scene: &mut Scene) {

    // Gets light sets
    let light_bundle = scene.light_bundle();
    let point_lights = &mut light_bundle.point_lights;
    let ambient_lights = &mut light_bundle.ambient_lights;
    let _directional_lights = &mut light_bundle.directional_lights;

    // Adds point light(s)
    let intensity = 40000.0;
    point_lights.lights.push(PointLight::new(
        [0.0, 100.0, 250.0],                   // Position
        [intensity, intensity, intensity],     // Color
        [1.0, 0.0, 1.0]                        // Attenuation
    ));
    point_lights.compute_radiuses(5.0/256.0);

    // Adds directional light(s)
    //let db = 255.0/255.0;
    //directional_lights.lights.push(DirectionalLight::new([0.0, -1.0, 0.0], [db, db, db]));       // White light pointing left (illuminates right site)

    // Adds ambient light(s)
    let ab = 2.0/255.0;
    ambient_lights.lights.push(AmbientLight::new([ab, ab, ab]));
}


fn create_tex_from_file(file_name: &str, device: &Device, queue: &Queue, format: TextureFormat) -> Texture {
    log::info!("Loading texture '{}'...", file_name);
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap();
    let tex = Texture::from_image(device, queue, &img, format, None);
    log::info!("Finished loading texture '{}'...", file_name);
    tex
}

fn create_box_model_and_instances(device: &Device, queue: &Queue) -> ModelInstanceSet {

    // Creates texture from image
    let diffuse_tex = create_tex_from_file("assets/cubemap/diffuse.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let emissive_tex = create_tex_from_file("assets/cubemap/emissive.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let normal_tex = create_tex_from_file("assets/cubemap/normal.png", device, queue, TextureFormat::Rgba8Unorm);
    let specular_tex = create_tex_from_file("assets/cubemap/specular.png", device, queue, TextureFormat::Rgba8Unorm);
    let gloss_tex = create_tex_from_file("assets/cubemap/gloss.png", device, queue, TextureFormat::Rgba8Unorm);
    let material = MaterialBuilder::new()
        .diffuse(diffuse_tex)
        .emissive(emissive_tex)
        .normal(normal_tex)
        .specular(specular_tex)
        .gloss(gloss_tex)
        .build(&device);

    // Creates cube model
    let model = Model {
        meshes: vec![Mesh::cube(&device, Color::WHITE, Vector3::new(100.0, 100.0, 100.0))],
        materials: vec![material],
        associations: vec![(0, 0)]
    };

    // Joins model with instance data and returns
    let mut mis = ModelInstanceSet::new(device, model, 4);
    mis
        .push(ModelInstance::new(Matrix4::identity().translate(Vector3::new(100.0, 0.0, 0.0))))
        .push(ModelInstance::new(Matrix4::identity().translate(Vector3::new(-100.0, 0.0, 0.0))))
        .push(ModelInstance::new(Matrix4::identity().translate(Vector3::new(-100.0, 0.0, 0.0))))
        .push(ModelInstance::new(Matrix4::identity()
            .translate(Vector3::new(0.0, 100.0, 0.0))
            .rotate_degrees(Vector3::new(1.0, 0.0, 0.0).normalize(), 45.0)
            .rotate_degrees(Vector3::new(0.0, 0.0, 1.0).normalize(), 45.0)
            .into()
        ));
    mis.flush(queue);
    mis
}

fn create_floor_and_instances(device: &Device, queue: &Queue) -> ModelInstanceSet {

    // Creates texture from image
    let diffuse_tex = create_tex_from_file("assets/cubemap/wood_diffuse.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let specular_tex = create_tex_from_file("assets/cubemap/wood_specular.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let gloss_tex = create_tex_from_file("assets/cubemap/wood_gloss.png", device, queue, TextureFormat::Rgba8UnormSrgb);
    let normal_tex = create_tex_from_file("assets/cubemap/wood_normal.png", device, queue, TextureFormat::Rgba8Unorm);
    let material = MaterialBuilder::new()
        .diffuse(diffuse_tex)
        .specular(specular_tex)
        .gloss(gloss_tex)
        .normal(normal_tex)
        .build(&device);

    // Creates cube model
    let model = Model {
        meshes: vec![Mesh::cube(&device, Color::WHITE, Vector3::new(100.0, 100.0, 100.0))],
        materials: vec![material],
        associations: vec![(0, 0)]
    };

    // Joins model with instance data and returns
    let mut mis = ModelInstanceSet::new(&device, model, 1);
    mis.push(ModelInstance::new(Matrix4::identity()
        .translate(Vector3::new(0.0, -200.0, -0.0))
        .scale(Vector3::new(10.0, 1.0, 10.0))
        .into()
    ));
    mis.flush(queue);
    mis
}
