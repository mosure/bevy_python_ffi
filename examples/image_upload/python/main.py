import time

import bevy_python_image_upload


bevy_python_image_upload.initialize()


data = bytearray([
    255, 0, 0, 255,
    0, 255, 0, 255,
    0, 0, 255, 255,
    255, 255, 255, 255,
])

width = 2
height = 2
format = bevy_python_image_upload.Format.Rgba8

bevy_python_image_upload.upload_image(
    data,
    format,
    width,
    height,
    "test_image",
)


bevy_python_image_upload.main(False)
