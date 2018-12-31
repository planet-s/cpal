#![allow(dead_code)]

use std::{fs, mem, slice};
use std::io::Write;

use CreationError;
use DefaultFormatError;
use Format;
use FormatsEnumerationError;
use SampleFormat;
use SampleRate;
use StreamData;
use SupportedFormat;
use UnknownTypeOutputBuffer;

pub struct EventLoop;

impl EventLoop {
    #[inline]
    pub fn new() -> EventLoop {
        EventLoop
    }

    #[inline]
    pub fn run<F>(&self, mut callback: F) -> !
        where F: FnMut(StreamId, StreamData)
    {
        println!("cpal: run");
        let mut audio = fs::OpenOptions::new().write(true).open("audio:").expect("failed to open audio:");
        loop {
            //TODO: Use events to allow for multiple streams
            let mut buffer = [0i16; 1024];
            callback(StreamId, StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(
                    ::OutputBuffer {
                        target: Some(
                            OutputBuffer(&mut buffer)
                        )
                    }
                )
            });
            let buffer_bytes = unsafe { slice::from_raw_parts(
                buffer.as_ptr() as *const u8,
                mem::size_of_val(&buffer)
            ) };
            audio.write(buffer_bytes).expect("failed to write to audio:");
        }
    }

    #[inline]
    pub fn build_input_stream(&self, device: &Device, format: &Format) -> Result<StreamId, CreationError> {
        println!("cpal: build_input_stream({:?}, {:?})", device, format);
        Ok(StreamId)
    }

    #[inline]
    pub fn build_output_stream(&self, device: &Device, format: &Format) -> Result<StreamId, CreationError> {
        println!("cpal: build_output_stream({:?}, {:?})", device, format);
        Ok(StreamId)
    }

    #[inline]
    pub fn destroy_stream(&self, stream: StreamId) {
        println!("cpal: destroy_stream({:?})", stream);
    }

    #[inline]
    pub fn play_stream(&self, stream: StreamId) {
        println!("cpal: play_stream({:?})", stream);
    }

    #[inline]
    pub fn pause_stream(&self, stream: StreamId) {
        println!("cpal: pause_stream({:?})", stream);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamId;

#[derive(Default)]
pub struct Devices;

impl Iterator for Devices {
    type Item = Device;

    #[inline]
    fn next(&mut self) -> Option<Device> {
        None
    }
}

#[inline]
pub fn default_input_device() -> Option<Device> {
    Some(Device)
}

#[inline]
pub fn default_output_device() -> Option<Device> {
    Some(Device)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device;

impl Device {
    #[inline]
    pub fn supported_input_formats(&self) -> Result<SupportedInputFormats, FormatsEnumerationError> {
        Ok(SupportedInputFormats(Some(SupportedFormat {
            channels: 2,
            min_sample_rate: SampleRate(44100),
            max_sample_rate: SampleRate(44100),
            data_type: SampleFormat::I16
        })))
    }

    #[inline]
    pub fn supported_output_formats(&self) -> Result<SupportedOutputFormats, FormatsEnumerationError> {
        Ok(SupportedOutputFormats(Some(SupportedFormat {
            channels: 2,
            min_sample_rate: SampleRate(44100),
            max_sample_rate: SampleRate(44100),
            data_type: SampleFormat::I16
        })))
    }

    #[inline]
    pub fn default_input_format(&self) -> Result<Format, DefaultFormatError> {
        Ok(Format {
            channels: 2,
            sample_rate: SampleRate(44100),
            data_type: SampleFormat::I16,
        })
    }

    #[inline]
    pub fn default_output_format(&self) -> Result<Format, DefaultFormatError> {
        Ok(Format {
            channels: 2,
            sample_rate: SampleRate(44100),
            data_type: SampleFormat::I16,
        })
    }

    #[inline]
    pub fn name(&self) -> String {
        "redox".to_owned()
    }
}

pub struct SupportedInputFormats(Option<SupportedFormat>);
pub struct SupportedOutputFormats(Option<SupportedFormat>);

impl Iterator for SupportedInputFormats {
    type Item = SupportedFormat;

    #[inline]
    fn next(&mut self) -> Option<SupportedFormat> {
        self.0.take()
    }
}

impl Iterator for SupportedOutputFormats {
    type Item = SupportedFormat;

    #[inline]
    fn next(&mut self) -> Option<SupportedFormat> {
        self.0.take()
    }
}

pub struct InputBuffer<'a, T: 'a>(&'a [T]);

impl<'a, T> InputBuffer<'a, T> {
    #[inline]
    pub fn buffer(&self) -> &[T] {
        self.0
    }

    #[inline]
    pub fn finish(self) {
    }
}

pub struct OutputBuffer<'a, T: 'a>(&'a mut [T]);

impl<'a, T> OutputBuffer<'a, T> {
    #[inline]
    pub fn buffer(&mut self) -> &mut [T] {
        self.0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn finish(self) {
    }
}
