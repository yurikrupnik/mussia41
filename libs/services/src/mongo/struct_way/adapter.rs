trait MediaPlayer {
    fn play(&self, audio_type: &str, file_name: &str);
}

struct AudioPlayer;

impl MediaPlayer for AudioPlayer {
    fn play(&self, audio_type: &str, file_name: &str) {
        if audio_type == "mp3" {
            println!("Playing mp3 file: {}", file_name);
        } else {
            let adapter = MediaAdapter::new(audio_type);
            adapter.play(audio_type, file_name);
        }
    }
}

trait AdvancedMediaPlayer {
    fn play_vlc(&self, file_name: &str);
    fn play_mp4(&self, file_name: &str);
}

struct VlcPlayer;

impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: &str) {
        println!("Playing vlc file: {}", file_name);
    }

    fn play_mp4(&self, _file_name: &str) {
        // Do nothing
    }
}

struct Mp4Player;

impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, _file_name: &str) {
        // Do nothing
    }

    fn play_mp4(&self, file_name: &str) {
        println!("Playing mp4 file: {}", file_name);
    }
}

struct MediaAdapter {
    advanced_player: Box<dyn AdvancedMediaPlayer>,
}

impl MediaAdapter {
    fn new(audio_type: &str) -> Self {
        let advanced_player: Box<dyn AdvancedMediaPlayer> = match audio_type {
            "vlc" => Box::new(VlcPlayer),
            "mp4" => Box::new(Mp4Player),
            _ => panic!("Invalid media type"),
        };

        MediaAdapter { advanced_player }
    }
}

impl MediaPlayer for MediaAdapter {
    fn play(&self, audio_type: &str, file_name: &str) {
        match audio_type {
            "vlc" => self.advanced_player.play_vlc(file_name),
            "mp4" => self.advanced_player.play_mp4(file_name),
            _ => panic!("Invalid media type"),
        }
    }
}
