use gst::{MessageType, prelude::*};

fn tutorial_main() {
    // Initialize GStreamer
    gst::init().unwrap();

    let uri = "https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm";

    // Create the elements
    let source = gst::ElementFactory::make("uridecodebin")
        .name("source")
        // Set the URI to play
        .property("uri", uri)
        .build()
        .expect("Could not create uridecodebin element.");
    let audio_convert = gst::ElementFactory::make("audioconvert")
        .name("convert")
        .build()
        .expect("Could not create convert element.");
    let audio_resample = gst::ElementFactory::make("audioresample")
        .name("resample")
        .build()
        .expect("Could not create resample element.");
    let audio_sink = gst::ElementFactory::make("autoaudiosink")
        .name("sink")
        .build()
        .expect("Could not create sink element.");
    let video_convert = gst::ElementFactory::make("videoconvert")
        .name("video_convert")
        .build()
        .expect("Could not create sink element.");
    let video_sink = gst::ElementFactory::make("autovideosink")
        .name("video_sink")
        .build()
        .expect("Could not create sink element.");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::with_name("test-pipeline");

    // Build the pipeline Note that we are NOT linking the source at this
    // point. We will do it later.
    pipeline
        .add_many([
            &source,
            &audio_convert,
            &audio_resample,
            &audio_sink,
            &video_convert,
            &video_sink,
        ])
        .unwrap();
    gst::Element::link_many([&audio_convert, &audio_resample, &audio_sink])
        .expect("Elements could not be linked.");
    gst::Element::link_many([&video_convert, &video_sink]).expect("Elements could not be linked.");

    // Connect the pad-added signal
    source.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());

        src.downcast_ref::<gst::Bin>()
            .unwrap()
            .debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pad-added");

        let new_pad_caps = src_pad
            .current_caps()
            .expect("Failed to get caps of new pad.");
        let new_pad_struct = new_pad_caps
            .structure(0)
            .expect("Failed to get first structure of caps.");
        let new_pad_type = new_pad_struct.name();

        let is_audio = new_pad_type.starts_with("audio/x-raw");
        let is_video = new_pad_type.starts_with("video/x-raw");

        if is_audio {
            let audio_sink_pad = audio_convert
                .static_pad("sink")
                .expect("Failed to get static sink pad from convert");
            if audio_sink_pad.is_linked() {
                println!("We are already linked. Ignoring.");
                return;
            }

            if src_pad.link(&audio_sink_pad).is_err() {
                println!("Type is {new_pad_type} but link failed.");
            } else {
                println!("Link succeeded (type {new_pad_type}).");
            }
        }

        if is_video {
            let video_sink_pad = video_convert
                .static_pad("sink")
                .expect("Failed to get static sink pad from convert");
            if video_sink_pad.is_linked() {
                println!("We are already linked. Ignoring.");
                return;
            }

            if src_pad.link(&video_sink_pad).is_err() {
                println!("Type is {new_pad_type} but link failed.");
            } else {
                println!("Link succeeded (type {new_pad_type}).");
            }
        }
    });

    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until State Change, error, or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed_filtered(
        gst::ClockTime::NONE,
        &[
            MessageType::StateChanged,
            MessageType::Error,
            MessageType::Eos,
        ],
    ) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?} {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed.src().map(|s| s == &pipeline).unwrap_or(false) {
                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current()
                    );
                }
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    tutorial_main();
}
