use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        atomic::{
            AtomicBool,
            Ordering,
        },
        mpsc::{
            self,
            Sender,
            Receiver,
        },
    },
    thread,
};

use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d,
        TextureDescriptor,
        TextureDimension,
        TextureFormat,
        TextureUsages,
    },
};
use once_cell::sync::OnceCell;
use pyo3::{
    prelude::*,
    exceptions::PyValueError,
    types::PyByteArray,
};


#[derive(Clone, Debug)]
#[pyclass]
pub enum Format {
    R8,
    Rg8,
    Rgba8,
    Bgra8,
    R32f,
    Rg32f,
    Rgba32f,
}

impl From<Format> for TextureFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::R8 => TextureFormat::R8Unorm,
            Format::Rg8 => TextureFormat::Rg8Unorm,
            Format::Rgba8 => TextureFormat::Rgba8Unorm,
            Format::Bgra8 => TextureFormat::Bgra8Unorm,
            Format::R32f => TextureFormat::R32Float,
            Format::Rg32f => TextureFormat::Rg32Float,
            Format::Rgba32f => TextureFormat::Rgba32Float,
        }
    }
}


static IMAGE_RECEIVER: OnceCell<Arc<Mutex<Receiver<ImagePayload>>>> = OnceCell::new();
static IMAGE_SENDER: OnceCell<Sender<ImagePayload>> = OnceCell::new();


#[pyfunction]
pub fn initialize() -> PyResult<()> {
    let (
        sender,
        receiver,
    ) = mpsc::channel();

    IMAGE_RECEIVER.set(Arc::new(Mutex::new(receiver))).unwrap();
    IMAGE_SENDER.set(sender).unwrap();

    Ok(())
}


/// upload_image(bytes, format, width, height, label, /)
/// --
///
/// upload bytes to an image with the target label
#[pyfunction]
pub fn upload_image(
    bytes: &Bound<'_, PyByteArray>,
    format: Format,
    width: u32,
    height: u32,
    label: &str,
) -> PyResult<()> {
    if bytes.is_empty() {
        return Err(PyValueError::new_err("empty image data"));
    }

    let data = unsafe { bytes.as_bytes().to_vec() };
    let format: TextureFormat = format.into();

    let payload = ImagePayload {
        data,
        width,
        height,
        format,
        label: label.to_string(),
    };

    if IMAGE_SENDER.get().is_none() {
        return Err(PyValueError::new_err("bevy_python_ffi `initialize` not called"));
    }

    let sender = IMAGE_SENDER.get().unwrap();
    let result = sender.send(payload);

    if result.is_err() {
        return Err(PyValueError::new_err(format!("failed to send image payload: {}", result.err().unwrap())));
    }

    Ok(())
}


pub struct ImagePayload {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub label: String,
}

fn bevy_upload_image(
    payload: ImagePayload,
    images: &mut Assets<Image>,
) -> Handle<Image> {
    let ImagePayload {
        data,
        width,
        height,
        format,
        label: _,
     } = payload;

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let mut uploaded_image = Image {
        data,
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::COPY_SRC
                 | TextureUsages::COPY_DST
                 | TextureUsages::TEXTURE_BINDING
                 | TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        },
        ..Default::default()
    };
    uploaded_image.resize(size);

    images.add(uploaded_image)
}


#[derive(Resource, Default, Debug)]
pub struct UploadedImages {
    pub images: HashMap<String, Handle<Image>>,
}

#[derive(Debug, Clone, Event)]
pub struct ImageUploadEvent {
    pub image: Handle<Image>,
    pub label: String,
}

#[derive(Default)]
struct ImageUploadPlugin;
impl Plugin for ImageUploadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ImageUploadEvent>();
        app.init_resource::<UploadedImages>();
        app.add_systems(Update, receive_image_upload_events);
    }
}

fn receive_image_upload_events(
    mut images: ResMut<Assets<Image>>,
    mut image_upload_events: EventWriter<ImageUploadEvent>,
    mut uploaded_images: ResMut<UploadedImages>,
) {
    if IMAGE_RECEIVER.get().is_none() {
        return;
    }

    let receiver = IMAGE_RECEIVER.get().unwrap().lock().unwrap();
    for payload in receiver.try_iter() {
        let label = payload.label.clone();
        let image = bevy_upload_image(
            payload,
            &mut images,
        );

        // TODO: reuse existing image if label already exists

        uploaded_images.images.insert(label.clone(), image.clone());

        image_upload_events.send(ImageUploadEvent {
            image,
            label,
        });
    }
}



/// runs an app on a new thread to unblock python GIL, call from virtual main method
pub fn setup_and_run_app(
    mut app: App,
    new_thread: bool,
) {
    let ready = Arc::new(AtomicBool::new(false));

    let mut startup = {
        let ready = Arc::clone(&ready);

        move || {
            app.add_plugins(ImageUploadPlugin);

            ready.store(true, Ordering::Release);

            app.run();
        }
    };

    if new_thread {
        thread::spawn(startup);

        while !ready.load(Ordering::Acquire) {
            thread::yield_now();
        }

        return;
    }

    startup();
}

/// register bevy_python_ffi module
pub fn register_python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Format>()?;
    m.add_function(wrap_pyfunction!(initialize, m)?)?;
    m.add_function(wrap_pyfunction!(upload_image, m)?)?;

    Ok(())
}
