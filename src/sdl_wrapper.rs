use sdl2::{event::Event, AudioSubsystem, EventPump, Sdl, VideoSubsystem};

pub struct SdlWrapper {
    context: Sdl,
    video: VideoSubsystem,
    audio: AudioSubsystem,
    event_pump: EventPump,
}

impl SdlWrapper {
    pub fn new() -> SdlWrapper {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let audio_subsystem = sdl_context.audio().expect("Unable to init audio");
        let event_pump = sdl_context.event_pump().unwrap();
        SdlWrapper {
            context: sdl_context,
            video: video_subsystem,
            audio: audio_subsystem,
            event_pump,
        }
    }

    pub fn get_window_canvas(
        &self,
        title: &str,
        width: u32,
        height: u32,
    ) -> sdl2::render::Canvas<sdl2::video::Window> {
        let window = self
            .video
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();
        canvas
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }
}
