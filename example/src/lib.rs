#![cfg(target_os = "android")]

use trs_24::{android_logger, overture::*, types::*};

#[no_mangle]
pub fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default());

    // Creates an event loop for android platforms only.
    let event_loop = EventLoopBuilder::new().with_android_app(app).build();

    let duck = trs_24::renderer::Model::new(
        include_bytes!("../../assets/models/duck/scene.gltf"),
        include_bytes!("../../assets/models/duck/scene.bin"),
        include_bytes!("../../assets/models/duck/texture.png"),
    )
    .set_position(Vec3::new(0.5, -0.5, 0.0))
    .set_scale(Vec3::new(0.006, 0.006, 0.006))
    .set_rotation(-26.0, trs_24::types::RotAxis::Roll);

    let map = trs_24::renderer::Model::new(
        include_bytes!("../../assets/models/map/scene.gltf"),
        include_bytes!("../../assets/models/map/scene.bin"),
        include_bytes!("../../assets/models/map/texture.png"),
    )
    .set_position(Vec3::new(-0.5, 0.0, 0.0))
    .set_scale(Vec3::new(0.08, 0.08, 0.08));

    let textbox = trs_24::ui::Element::new(trs_24::types::ElementType::Shape(
        trs_24::ui::ShapeBuilder::new(trs_24::types::Shape::Square),
    ))
    .set_color(RGBA::new(0.5, 0.0, 1.0, 0.4))
    .set_position(Vec3::new(0.0, -0.74, 0.0))
    .set_scale(Vec3::new(0.7, 0.2, 0.5));

    let text = trs_24::ui::Element::new(trs_24::types::ElementType::Text(
        trs_24::ui::TextBuilder::new(
            "TRS_24 Demo".to_string(),
            include_bytes!("../../assets/fonts/Antonio-Bold.ttf"),
            60,
        ),
    ))
    .set_color(RGBA::new(1.0, 1.0, 1.0, 1.0))
    .set_scale(Vec3::new(0.002, 0.002, 0.002))
    .set_position(Vec3::new(-0.3, -0.8, 0.0));

    Scene::new(event_loop).run(
        RGBA::new(0.1, 0.1, 0.1, 1.0),
        vec![duck, map],
        vec![textbox, text],
    );
}
