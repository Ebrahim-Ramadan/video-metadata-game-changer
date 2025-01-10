use ffmpeg_next::{format, frame, software, util};
use std::process::Command;

fn main() {
    
    let input_video = "input_video.mp4";
    let thumbnail_image = "new_thumbnail.jpg";
    let output_video = "output_video.mp4";

    
    Command::new("ffmpeg")
        .args(&["-i", input_video, "-ss", "00:00:01.000", "-vframes", "1", thumbnail_image])
        .status()
        .expect("Failed to generate thumbnail");

    
    update_metadata(input_video, thumbnail_image, output_video).expect("Failed to update metadata");
}

fn update_metadata(input_video: &str, thumbnail_image: &str, output_video: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut input = format::input(&input_video)?;

    
    let mut thumbnail = frame::Video::empty();
    

    
    let scaler = software::scaling::context::Context::get(
        thumbnail.format(),
        thumbnail.width(),
        thumbnail.height(),
        format::Pixel::RGBA,
        120,
        120,
        software::scaling::flag::BILINEAR,
    )?;

    
    let mut scaled_thumbnail = frame::Video::empty();
    scaler.run(&thumbnail, &mut scaled_thumbnail)?;

    
    let mut output = format::output(&output_video)?;
    output.set_metadata(input.metadata().to_owned());
    output.add_stream(&input.stream(0).unwrap())?;
    
    
    output.set_metadata(Some(util::dictionary::Owned::new().set("thumbnail", &scaled_thumbnail)));

    
    output.write_header()?;
    output.write_trailer()?;

    Ok(())
}