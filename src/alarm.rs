use std::sync::{Arc, Mutex};
use std::thread;

use chrono::Duration;

use rodio::cpal::FromSample;
use rodio::source::{self, Empty, Repeat, SineWave, Source};
use rodio::{cpal::Stream, Decoder, OutputStream};
use rodio::{dynamic_mixer, OutputStreamHandle, Sample, Sink};

pub struct Alarm {
    duration: Duration,
    should_stop: Arc<Mutex<bool>>,
}

impl Alarm {
    pub fn new(duration: Duration) -> Alarm {
        Alarm {
            should_stop: Arc::new(Mutex::new(true)),
            duration,
        }
    }
    pub fn play(&mut self) {
        let duration2 = self.duration.to_std().unwrap();
        let should_stop = self.should_stop.clone();
        tokio::task::spawn_blocking(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
            let a = AlarmSource {
                source1: SineWave::new(1220.0),
                source2: SineWave::new(1.0).amplify(0.001_f32),
                num_sample: 0,
                rate: 10000,
            };
            let infinite = a.repeat_infinite().take_duration(duration2);
            sink.append(infinite);
            *should_stop.lock().unwrap() = false;
            while !*should_stop.lock().unwrap() {
                let _ = tokio::time::sleep(tokio::time::Duration::from_millis(100));
            }
        });
    }
    pub fn stop(&self) {
        *self.should_stop.lock().unwrap() = true;
    }
}

#[derive(Debug, Clone)]
struct AlarmSource<S1, S2>
where
    S1: Source<Item = f32> + Send + 'static,
    S2: Source<Item = f32> + Send + 'static,
{
    source1: S1,
    source2: S2,
    rate: usize,
    num_sample: usize,
}

impl<S1, S2> AlarmSource<S1, S2>
where
    S1: Source<Item = f32> + Send + 'static,
    S2: Source<Item = f32> + Send + 'static,
{
    fn get_current_source_index(&self) -> usize {
        let AlarmSource {
            num_sample, rate, ..
        } = self;
        let eighth = *num_sample / rate % 8;
        if eighth == 0 || eighth == 6 {
            0
        } else {
            1
        }
    }
}

impl<S1, S2> Source for AlarmSource<S1, S2>
where
    S1: Source<Item = f32> + Send + 'static,
    S2: Source<Item = f32> + Send + 'static,
{
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl<S1, S2> Iterator for AlarmSource<S1, S2>
where
    S1: Source<Item = f32> + Send + 'static,
    S2: Source<Item = f32> + Send + 'static,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.get_current_source_index() == 0 {
            self.source1.next()
        } else {
            self.source2.next()
        };
        self.num_sample += 1;
        next
    }
}
