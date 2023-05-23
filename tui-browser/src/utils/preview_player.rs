use std::{
    error::Error,
    io::{self, Cursor},
};

use rodio::{Decoder, OutputStream, Sink};

pub struct Preview {
    pub state: PreviewState,
    pub sink: Sink,
    _stream: OutputStream,
    cursor: Cursor<Vec<u8>>,
    volume: f32,
}

#[derive(PartialEq)]
pub enum PreviewState {
    Playing,
    Paused,
    Stopped,
}

impl Preview {
    pub async fn new(url: &String) -> Result<Preview, Box<dyn Error>> {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let cursor = get_preview_audio(url).await;

        match cursor {
            Ok(cursor) => {
                sink.set_volume(0.1);

                Ok(Preview {
                    state: PreviewState::Stopped,
                    sink,
                    _stream,
                    cursor,
                    volume: 0.1,
                })
            }
            Err(_) => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get preview audio",
            ))),
        }
    }

    pub fn play(&mut self) {
        self.sink.append(Decoder::new(self.cursor.clone()).unwrap());
        self.state = PreviewState::Playing;
    }

    pub fn resume(&mut self) {
        self.sink.play();
        self.state = PreviewState::Playing;
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.state = PreviewState::Paused;
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.state = PreviewState::Stopped;
    }

    pub fn inc_vol(&mut self) {
        if self.volume == 1.0 {
            return;
        };

        self.volume += 0.02;
        self.sink.set_volume(self.volume);
    }

    pub fn dec_vol(&mut self) {
        if self.volume == 0.00 {
            return;
        }
        self.volume -= 0.02;
        self.sink.set_volume(self.volume);
    }
}

async fn get_preview_audio(url: &String) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(_) => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get preview audio",
            )));
        }
    };
    Ok(Cursor::new((response.bytes().await.unwrap()).to_vec()))
}
