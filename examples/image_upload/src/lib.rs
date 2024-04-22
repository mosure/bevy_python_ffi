use bevy::prelude::*;
use bevy_python_ffi::{
    ImageUploadEvent,
    register_python_module,
    setup_and_run_app,
};
use pyo3::prelude::*;


#[pyfunction]
fn main(new_thread: bool) {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup_camera);
    app.add_systems(Update, receive_image_upload_events);

    setup_and_run_app(app, new_thread);
}

#[derive(Component, Default)]
struct ImageViewer;

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    })
    .with_children(|builder| {
        builder.spawn(ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .insert(ImageViewer);
    });

    commands.spawn(Camera2dBundle::default());
}

fn receive_image_upload_events(
    mut events: EventReader<ImageUploadEvent>,
    mut viewer: Query<
        &mut UiImage,
        With<ImageViewer>,
    >,
) {
    if events.is_empty() {
        return;
    }

    for event in events.read() {
        println!("received image upload event: {:?}", event);

        let mut ui_image = viewer.single_mut();
        ui_image.texture = event.image.clone();
    }
}



#[pymodule]
fn bevy_python_image_upload(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_python_module(m);

    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}
