extern crate ffmpeg_next as ffmpeg;

use std::{fs::File, time::Duration};
use color_eyre::Report;
use ffmpeg::{codec::Id, media};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
struct Format {
    name: String,
    description: String,
    extensions: Vec<String>,
    mime_types: Vec<String>
}

impl From<ffmpeg::format::Input> for Format {
    fn from(format: ffmpeg::format::Input) -> Self {
        Format {
            description: format.description().to_owned(),
            extensions: format.extensions().into_iter().map(|x| x.to_owned()).collect(),
            mime_types: format.mime_types().into_iter().map(|x| x.to_owned()).collect(),
            name: format.name().to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
struct Codec {
    id: Id,
    codec_type: media::Type,
}

impl From<ffmpeg::codec::context::Context> for Codec {
    fn from(c: ffmpeg::codec::context::Context) -> Self {
        Codec {
            id: c.id(),
            codec_type: c.medium()
        }
    }
}

struct FrameIterator{
    file: File
}

struct Frame {
    timestamp: Duration
}

impl Iterator for FrameIterator {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn frame_iterator(file: File) -> FrameIterator {
    FrameIterator {
        file
    }
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    color_eyre::install()?;
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();

    Ok(())
}

fn main() -> Result<(), Report> {
    setup()?;
    ffmpeg::init()?;
    let mut i = ffmpeg::format::input(&"data/movie.mkv")?;
    dbg!(i.nb_chapters());
    for chapter in i.chapters() {
        dbg!(chapter.metadata());
    }
    let f = Format::from(i.format());
    dbg!(f);

    for (k,v) in i.metadata().iter() {
        println!("{}: {}", k, v);
    }

    for s in i.streams() {
        let c = Codec::from(s.codec());
        dbg!(c);
    }


    println!(
        "duration (seconds): {:.2}",
        i.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
    );

    let method =  unsafe {*i.as_ptr()}.duration_estimation_method;
    println!("\tduration_estimation_method: {:?}", method);


    if let Some(stream) = i.streams().best(media::Type::Video) {
        let c = Codec::from(stream.codec());
        dbg!(c);

        println!("\ttime_base: {}", stream.time_base());
        println!("\tstart_time: {}", stream.start_time());
        println!("\trate: {}", stream.rate());
        let index =  stream.index();
        println!("stream index {}:",index);
        let mut decoder = stream.codec().decoder().video()?;

        let mut score_array = vec![];
        let mut previous_frame: Option<ffmpeg::frame::Video> = None;
        for (s, mut p) in i.packets() {
            if s.index() != index {
                continue;
            }
            let mut frame = ffmpeg::frame::Video::empty();
            decoder.send_packet(&p)?;

            if let Err(res) = decoder.receive_frame(&mut frame) {
                continue;
            }
            dbg!(frame.kind());
            dbg!(frame.pts());
            dbg!(frame.coded_number());
            dbg!(frame.display_number());

            if let Some(pf) = previous_frame.as_ref() {
                let score = score_frame(pf, &frame);
                score_array.push(score);
                break;
            }
            previous_frame = Some(frame);
        }
        dbg!(score_array);
    } else {
        println!("No video stream found")
    }
    Ok(())
}

fn score_frame(previous_frame: &ffmpeg::frame::Video, frame: &ffmpeg::frame::Video) -> f64 {

    unsafe {
        dbg!(previous_frame.is_empty());
        dbg!(frame.is_empty());
    }
    dbg!(previous_frame.aspect_ratio());
    dbg!(frame.aspect_ratio());
    let h = previous_frame.height() as usize;
    let w = previous_frame.width()  as usize;

    dbg!(h, w);
    dbg!(previous_frame.planes());
    dbg!(previous_frame.format());
    dbg!(previous_frame.plane::<[u8; 3]>(0).len());
    let data = previous_frame.data(0);
    dbg!(data.len());

    return 0.0
}