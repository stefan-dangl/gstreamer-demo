# Gstreamer Demo

Ideas for demo project:
- Waveform modulator
    - GUI that allows to select different waveforms and audio properties. The waveform is shown graphically and the sound is outputted
    - Based on Basic Tutorial 6, 7
- Synthesizer that can be controlled by keyboard
- Custom Audio Sink
    - Takes frequency + 10 values that allow to define custom waveforms as input. The 10 values are evenly distributed and linearly interpolated. 


Notes
- Audio Visualization tool: Monoscope
    - [Playback tutorial 6: Audio visualization](https://gstreamer.freedesktop.org/documentation/tutorials/playback/custom-playbin-sinks.html?gi-language=c)
- Rust Tutorial Code: [Git](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/tutorials/src/bin/basic-tutorial-7.rs?ref_type=heads)