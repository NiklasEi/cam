use std::fs::create_dir_all;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, io};
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::FourCC;
use v4l::{context, Device};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("You need to pass me a directory to save the images in!")
    }
    let directory = &args[1];

    let devices = context::enum_devices();

    for dev in devices {
        println!("{}: {}", dev.index(), dev.name().unwrap());
    }
    let mut dev = Device::new(0).expect("Failed to open device");

    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = 1280;
    fmt.height = 960;
    fmt.fourcc = FourCC::new(b"MJPG");
    dev.set_format(&fmt).expect("Failed to write format");

    println!("Format in use:\n{}", fmt);
    let params = dev.params()?;
    println!("Active parameters:\n{}", params);

    create_dir_all(format!("output/{}/", directory))?;

    let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, 1)
        .expect("Failed to create buffer stream");

    let (buf, meta) = stream.next().unwrap();
    println!(
        "Buffer size: {}, seq: {}, timestamp: {}",
        buf.len(),
        meta.sequence,
        meta.timestamp
    );

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let img_buffer = image::load_from_memory(buf).unwrap();
    img_buffer
        .save(format!("output/{}/{}.png", directory, time))
        .unwrap();
    Ok(())
}
