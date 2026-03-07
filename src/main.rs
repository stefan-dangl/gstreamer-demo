use gst::{MessageType, prelude::*};

fn tutorial_main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the elements
    let source = gst::ElementFactory::make("videotestsrc")
        .name("source")
        .property_from_str("pattern", "ball")
        .property_from_str("background-color", "0x00ffff")
        .property_from_str("foreground-color", "0xff0000")
        .property_from_str("flip", "true")
        .build()
        .expect("Could not create source element.");

    let filter = gst::ElementFactory::make("vertigotv")
        .name("filter")
        .build()
        .expect("Could not create filter element");

    let videoconvert = gst::ElementFactory::make("videoconvert")
        .name("videoconvert")
        .build()
        .expect("Could not create videoconvert element");

    let sink = gst::ElementFactory::make("autovideosink")
        .name("sink")
        .build()
        .expect("Could not create sink element");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::with_name("test-pipeline");

    // Build the pipeline
    pipeline
        .add_many([&source, &filter, &videoconvert, &sink])
        .unwrap();
    source.link(&filter).expect("Elements could not be linked.");
    filter
        .link(&videoconvert)
        .expect("Elements could not be linked.");
    videoconvert
        .link(&sink)
        .expect("Elements could not be linked.");

    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed_filtered(
        gst::ClockTime::NONE,
        &[MessageType::Error, MessageType::Eos],
    ) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
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
