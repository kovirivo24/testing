//use std::{error::Error, io::stdin};
use engine::{Engine, GPUCamera, GPUSprite, Game};
use kira::{
    manager::{
        backend::DefaultBackend, // changed to default backend
        AudioManager,
        AudioManagerSettings,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
struct TestGame {
    //move some sctucts into here {
    camera: GPUCamera,
    audio_manager: AudioManager,
    sound_data: StaticSoundData,
}

struct Resources{
    bgm: StaticSoundData,
    jump: StaticSoundData,
}
#[async_trait::async_trait]
impl Game for TestGame {
    async fn init(&mut self, engine: &mut Engine) {
        //Creating our background image texture, by calling load texture
        let (img, _) = engine
            .load_texture("scene2d/src/background.jpg", None)
            .expect("Couldn't load background");

        //Then we are adding this and behind the scenes it shoudl be creating a bind group and etc to display it.
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![GPUSprite {
                screen_region: [0.0, 0.0, 1024.0, 768.0],
                sheet_region: [0.0, 0.0, 1.0, 1.0],
            }],
            self.camera,
        );

        //Same thing but for our sprite
        let (tex_king, _) = engine
            .load_texture("scene2d/src/kiiiii.png", None)
            .expect("Couldn't load king img");

        //This sprite-group we would want to add obstacles and etc
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                GPUSprite {
                    screen_region: [400.0, 200.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [400.0 + 64.0, 200.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [400.0 + 128.0, 200.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [700.0 + 192.0, 200.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
            ],
            self.camera,
        );

        //This sprite group adds the left Player
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                //It's the 2 different sprites for king.png at 2 different locations
                GPUSprite {
                    screen_region: [32.0, 85.0, 64.0, 64.0],
                    sheet_region: [32.0 / 64.0, 48.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0],
                },
            ],
            self.camera,
        );

        //This sprite group adds the right player
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                //It's the 2 different sprites for king.png at 2 different locations
                GPUSprite {
                    screen_region: [750.0, 85.0, 64.0, 64.0],
                    sheet_region: [32.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0],
                },
            ],
            self.camera,
        );
        let _ = self.audio_manager.play(self.sound_data.clone());

        //72x88 (9 by 11 - so 8 each)
        let (font, _) = engine
            .load_texture("scene2d/src/font.png", None)
            .expect("Couldn't load background");

        //music engine

        //Then we are adding this and behind the scenes it shoudl be creating a bind group and etc to display it.
        //Overlay
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &font,
            vec![
                GPUSprite {
                    screen_region: [2.0, 728.0, 32.0, 32.0],
                    sheet_region: [0.0, 16.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0, 728.0, 32.0, 32.0],
                    sheet_region: [16.0 / 72.0, 0.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0 * 2.0, 728.0, 32.0, 32.0],
                    sheet_region: [40.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0 * 3.0, 728.0, 32.0, 32.0],
                    sheet_region: [64.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0 * 4.0, 728.0, 32.0, 32.0],
                    sheet_region: [32.0 / 72.0, 0.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0 * 4.0 + 16.0, 726.0, 32.0, 32.0],
                    sheet_region: [32.0 / 72.0, 72.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0 * 5.0 + 8.0, 728.0, 32.0, 32.0],
                    sheet_region: [64.0 / 72.0, 40.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0 * 6.0 + 8.0, 728.0, 32.0, 32.0],
                    sheet_region: [64.0 / 72.0, 40.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    screen_region: [2.0 + 32.0 * 7.0 + 8.0, 728.0, 32.0, 32.0],
                    sheet_region: [64.0 / 72.0, 40.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
            ],
            self.camera,
        );

        //Powerups - 5
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![GPUSprite {
                screen_region: [560.0, 90.0, 64.0, 64.0],
                sheet_region: [32.0 / 64.0, 0.0, 16.0 / 64.0, 16.0 / 64.0],
            }],
            self.camera,
        );

        let (img, _) = engine
            .load_texture("scene2d/src/titleScreenBackground.jpg", None)
            .expect("Couldn't load background");

        //Title Screen Background
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![GPUSprite {
                screen_region: [0.0, 0.0, 1024.0, 768.0],
                sheet_region: [0.0, 0.0, 1.0, 1.0],
            }],
            self.camera,
        );

        let (img, _) = engine
            .load_texture("scene2d/src/font.png", None)
            .expect("Couldn't load background");

        let starting_x = 200.0;
        let y_val = 600.0;
        //GAMENAME
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![
                GPUSprite {
                    //P
                    screen_region: [starting_x, y_val, 64.0, 64.0],
                    sheet_region: [48.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //L
                    screen_region: [starting_x + 66.0, y_val, 64.0, 64.0],
                    sheet_region: [16.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //A
                    screen_region: [starting_x + (66.0 * 2.0), y_val, 64.0, 64.0],
                    sheet_region: [0.0 / 72.0, 0.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //T
                    screen_region: [starting_x + (66.0 * 3.0), y_val, 64.0, 64.0],
                    sheet_region: [8.0 / 72.0, 16.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //F
                    screen_region: [starting_x + (66.0 * 4.0), y_val, 64.0, 64.0],
                    sheet_region: [40.0 / 72.0, 0.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //O
                    screen_region: [starting_x + (66.0 * 5.0), y_val, 64.0, 64.0],
                    sheet_region: [40.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //R
                    screen_region: [starting_x + (66.0 * 6.0), y_val, 64.0, 64.0],
                    sheet_region: [64.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //M
                    screen_region: [starting_x + (66.0 * 7.0), y_val, 64.0, 64.0],
                    sheet_region: [24.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //E
                    screen_region: [starting_x + (66.0 * 8.0), y_val, 64.0, 64.0],
                    sheet_region: [32.0 / 72.0, 0.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //R
                    screen_region: [starting_x + (66.0 * 9.0), y_val, 64.0, 64.0],
                    sheet_region: [64.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
            ],
            self.camera,
        );

        let (img, _) = engine
            .load_texture("scene2d/src/font.png", None)
            .expect("Couldn't load background");

        let starting_x = 234.0;
        let y_val = 500.0;
        //OPTIONS
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![
                GPUSprite {
                    //O
                    screen_region: [starting_x, y_val, 32.0, 32.0],
                    sheet_region: [40.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //P
                    screen_region: [starting_x + 34.0, y_val, 32.0, 32.0],
                    sheet_region: [48.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //T
                    screen_region: [starting_x + (34.0 * 2.0), y_val, 32.0, 32.0],
                    sheet_region: [8.0 / 72.0, 16.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //I
                    screen_region: [starting_x + (34.0 * 3.0), y_val, 32.0, 32.0],
                    sheet_region: [64.0 / 72.0, 0.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //O
                    screen_region: [starting_x + (34.0 * 4.0), y_val, 32.0, 32.0],
                    sheet_region: [40.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //N
                    screen_region: [starting_x + (34.0 * 5.0), y_val, 32.0, 32.0],
                    sheet_region: [32.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //S
                    screen_region: [starting_x + (34.0 * 6.0), y_val, 32.0, 32.0],
                    sheet_region: [0.0 / 72.0, 16.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
            ],
            self.camera,
        );

        let (img, _) = engine
            .load_texture("scene2d/src/font.png", None)
            .expect("Couldn't load background");

        let y_val = 400.0;
        //PLAY
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![
                GPUSprite {
                    //P
                    screen_region: [starting_x, y_val, 32.0, 32.0],
                    sheet_region: [48.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //L
                    screen_region: [starting_x + 34.0, y_val, 32.0, 32.0],
                    sheet_region: [16.0 / 72.0, 8.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //A
                    screen_region: [starting_x + (34.0 * 2.0), y_val, 32.0, 32.0],
                    sheet_region: [0.0 / 72.0, 0.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
                GPUSprite {
                    //Y
                    screen_region: [starting_x + (34.0 * 3.0), y_val, 32.0, 32.0],
                    sheet_region: [48.0 / 72.0, 16.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
                },
            ],
            self.camera,
        );

        let (img, _) = engine
            .load_texture("scene2d/src/font.png", None)
            .expect("Couldn't load background");

        //Select thing
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![GPUSprite {
                //>
                screen_region: [200.0, 500.0, 32.0, 32.0],
                sheet_region: [16.0 / 72.0, 80.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
            }],
            self.camera,
        );

        let (tex_king, _) = engine
            .load_texture("scene2d/src/background.jpg", None)
            .expect("Couldn't load king img");

        //This sprite-group we would want to add obstacles and etc
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                GPUSprite {
                    screen_region: [300.0, 0.0, 0.0, 0.0], //[300.0, 0.0, 64.0, 85.0],
                    sheet_region: [300.0 / 1600.0, 800.0 / 1200.0, 30.0 / 1600.0, 30.0 / 1200.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [800.0, 0.0, 0.0, 0.0], //[800.0, 0.0, 64.0, 85.0],
                    sheet_region: [300.0 / 1600.0, 800.0 / 1200.0, 30.0 / 1600.0, 30.0 / 1200.0], //[0.0, 0.5, 0.5, 0.5]
                },
            ],
            self.camera,
        );

        let (tex_king, _) = engine
            .load_texture("scene2d/src/kiiiii.png", None)
            .expect("Couldn't load king img");

        //This sprite-group we would want to add obstacles and etc
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                GPUSprite {
                    screen_region: [400.0, 200.0, 0.0, 0.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [400.0 + 64.0, 200.0, 0.0, 0.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [400.0 + 128.0, 200.0, 0.0, 0.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
            ],
            self.camera,
        );

        let (tex_king, _) = engine
            .load_texture("scene2d/src/kiiiii.png", None)
            .expect("Couldn't load king img");

        //This sprite-group we would want to add obstacles and etc
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                GPUSprite {
                    screen_region: [150.0, 72.0, 0.0, 0.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [150.0, 130.0, 0.0, 0.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [850.0, 72.0, 0.0, 0.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [850.0, 130.0, 0.0, 0.0],
                    sheet_region: [0.0, 0.0, 16.0 / 64.0, 16.0 / 64.0], //[0.0, 0.5, 0.5, 0.5]
                },
            ],
            self.camera,
        );

        // OPTIONS MENU
        let (img, _) = engine
        .load_texture("scene2d/src/optionBackground.png", None)
        .expect("Couldn't load background");

        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![GPUSprite {
                screen_region: [0.0, 0.0, 0.0, 0.0],
                sheet_region: [0.0, 0.0, 1.0, 1.0],
            }],
            self.camera,
        );

        let (img, _) = engine
            .load_texture("scene2d/src/font.png", None)
            .expect("Couldn't load background");

        //more select things
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![
            GPUSprite {
                //> High Contrast
                screen_region: [40.0, 425.0, 0.0, 0.0],
                sheet_region: [16.0 / 72.0, 80.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
            },
            GPUSprite {
                //> Audio Cues
                screen_region: [40.0, 375.0, 0.0, 0.0],
                sheet_region: [16.0 / 72.0, 80.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
            },
            GPUSprite {
                //> Left Keyboard
                screen_region: [40.0, 325.0, 0.0, 0.0],
                sheet_region: [16.0 / 72.0, 80.0 / 88.0, 8.0 / 72.0, 8.0 / 88.0],
            }],
            self.camera,
        );
    }

    fn update(&mut self, engine: &mut Engine) {
        //put here
        engine.sprites.set_camera_all(&engine.gpu, self.camera);
        engine
            .sprites
            .refresh_sprites(&engine.gpu, 0, 0..(engine.sprites.get_sprites(0).len()));
    }
}

fn main() {
    let camera = GPUCamera {
        screen_pos: [0.0, 0.0],
        screen_size: [1024.0, 768.0],
    };
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let sound_data = StaticSoundData::from_file(
        "scene2d/src/musiccontent/test.mp3",
        StaticSoundSettings::default(),
    )
    .unwrap();
    Engine::start(
        event_loop,
        window,
        TestGame {
            camera,
            audio_manager: manager,
            sound_data,
        },
    );
}
