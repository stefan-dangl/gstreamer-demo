use anyhow::Error;
use gst::prelude::*;

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer
    gst::init()?;

    // Build the pipeline
    let pipeline = gst::parse::launch(
        "playbin uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm",
    )?;

    // Create elements that go inside the sink bin
    let video_effect = gst::ElementFactory::make("solarize")
        .name("video_effect")
        .build()
        .expect("Could not create video effect element.");
    let convert = gst::ElementFactory::make("videoconvert")
        .name("convert")
        .build()
        .expect("Could not create videoconvert element.");
    let sink = gst::ElementFactory::make("autovideosink")
        .name("audio_sink")
        .build()
        .expect("Could not create autovideosink element.");

    // Create the sink bin, add the elements and link them
    let bin = gst::Bin::with_name("video_sink_bin");
    bin.add_many([&video_effect, &convert, &sink]).unwrap();
    gst::Element::link_many([&video_effect, &convert, &sink]).expect("Failed to link elements.");

    let pad = video_effect
        .static_pad("sink")
        .expect("Failed to get a static pad from video_effect.");
    let ghost_pad = gst::GhostPad::builder_with_target(&pad).unwrap().build();
    ghost_pad.set_active(true)?;
    bin.add_pad(&ghost_pad)?;

    // Configure the equalizer
    // equalizer.set_property("band0", 12.0);
    // equalizer.set_property("band1", -24.0);
    // equalizer.set_property("band2", 12.0);

    pipeline.set_property("video-sink", &bin);

    // Set to PLAYING
    pipeline.set_state(gst::State::Playing)?;

    // Wait until an EOS or error message appears
    let bus = pipeline.bus().unwrap();
    let _msg = bus.timed_pop_filtered(
        gst::ClockTime::NONE,
        &[gst::MessageType::Error, gst::MessageType::Eos],
    );

    // Clean up
    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match tutorial_main() {
        Ok(_) => {}
        Err(err) => eprintln!("Failed: {err}"),
    };
}
