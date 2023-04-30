/// The module that implements audio support for our app

use std::{
    io::Cursor
};

use rodio::{
    Decoder,
    OutputStream,
    OutputStreamHandle,
    source::{Source, SamplesConverter},
    Sink
};

use crate::{
    errors::AudioError,
    static_data
};


pub type Volume = f32;
type RawData = &'static[u8];
type SampleType = f32;
type Sauce = SamplesConverter<Decoder<Cursor<RawData>>, SampleType>;


/// An audio manager
/// this only exist to keep references to all 3 main components in one place
/// and drop them at the same time
pub struct AudioManager {
    stream: OutputStream,
    handle: OutputStreamHandle,
    sink: Sink
}

impl AudioManager {
    /// Creates a new AudioManager
    pub fn new(stream: OutputStream, handle: OutputStreamHandle, sink: Sink) -> Self {
        return Self { stream, handle, sink }
    }

    // FIXME: Impl Default (if possible, we're returning Result here)
    pub fn new_default() -> Result<Self, AudioError> {
        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;
        return Ok(
            Self::new(stream, handle, sink)
        );
    }

    /// Returns a reference to Sink of this AudioManager
    pub fn get_sink(&self) -> &Sink {
        return &self.sink;
    }

    /// Returns a reference to OutputStream of this AudioManager
    #[allow(dead_code)]
    pub fn get_stream(&self) -> &OutputStream {
        return &self.stream;
    }

    /// Returns a reference to OutputStreamHandle of this AudioManager
    #[allow(dead_code)]
    pub fn get_handle(&self) -> &OutputStreamHandle {
        return &self.handle;
    }

    /// Appends a new Source to play in this audio manager
    pub fn append_source(&self, source: Sauce, repeat: bool) {
        if repeat {
            self.get_sink().append(source.repeat_infinite());
        }
        else {
            self.get_sink().append(source);
        }
    }

    /// Builds Source from raw data, then appends to the queue of this audio manager
    pub fn append_raw(&self, data: RawData, repeat: bool) -> Result<(), AudioError> {
        self.append_source(get_source_from_raw(data)?, repeat);
        return Ok(());
    }

    /// Stops the music and clears memory
    /// TODO: Impl Drop
    pub fn stop(self) {
        drop(self);
    }

    /// Pauses the audio
    #[allow(dead_code)]
    pub fn pause(&self) {
        self.get_sink().pause();
    }

    /// Unpauses the audio
    #[allow(dead_code)]
    pub fn unpause(&self) {
        self.get_sink().play();
    }

    /// Returns current volume
    pub fn get_volume(&self) -> Volume {
        return self.get_sink().volume()
    }

    /// Sets new volume
    pub fn set_volume(&self, volume: Volume) {
        self.get_sink().set_volume(volume);
    }
}


fn get_source_from_raw(data: RawData) -> Result<Sauce, AudioError> {
    let buf = Cursor::new(data);
    let decoder = Decoder::new(buf)?.convert_samples::<SampleType>();

    return Ok(decoder);
}

/// Starts playing the main theme
/// To stop the audio, use AudioManager.stop()
pub fn play_theme() -> Result<AudioManager, AudioError> {
    let manager = AudioManager::new_default()?;
    manager.append_raw(static_data::INSTALLER_THEME_DATA, true)?;

    return Ok(manager);
}
