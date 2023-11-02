use crate::{input, sprite::SpriteRender, Game, WGPU};
//use std::thread;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use kira::{
    manager::{
        backend::DefaultBackend, // changed to default backend
        AudioManager,
        AudioManagerSettings,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
//TODO IF JUMPING YOU CAN"T JUMP AGAIN UNTIL A CERTAIN PERIOD OF TIME WHY IS THAT

pub struct Engine {
    pub gpu: WGPU,
    pub sprites: SpriteRender,
    pub input: input::Input,
    pub is_jumping: bool,
    pub leftis_jumping: bool,
    pub velocity_y: f32,
    pub leftvelocity_y: f32,
    pub gravity: f32,
    pub score: usize,
    pub single_player: bool,
    pub sub_score: usize,
    pub audio_cues: bool,
    pub left_keyboard: bool,
    pub high_contrast: bool,
    pub audio: AudioManager,
}

pub struct Keyboard {
    pub left: VirtualKeyCode,
    pub right: VirtualKeyCode,
    pub up: VirtualKeyCode,
    pub down: VirtualKeyCode,
}

impl Engine {
    pub fn start(event_loop: EventLoop<()>, window: Window, game: impl Game + 'static) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
            // On native, we just want to wait for `run` to finish.
            pollster::block_on(Self::run(event_loop, window, game));
        }
        #[cfg(target_arch = "wasm32")]
        {
            // On web things are a little more complicated.
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            use winit::platform::web::WindowExtWebSys;
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
            // Now we use the browser's runtime to spawn our async run function.
            wasm_bindgen_futures::spawn_local(run(event_loop, window));
        }
    }
    async fn run(event_loop: EventLoop<()>, window: Window, mut game: impl Game + 'static) {
        let gpu = WGPU::new(&window).await;
        let sprites = SpriteRender::new(&gpu);
        let is_jumping = false;
        let leftis_jumping = false;
        let velocity_y = 0.0;
        let leftvelocity_y = 0.0;
        let gravity = -0.8; // Adjust this. Negative as it will pull the sprite down.
        let score = 0;
        let input = input::Input::default();
        let single_player = false;
        let sub_score = 0;
        let mut already_removed_multi: bool = false;
        let mut options_shown = false;
        let audio_cues = false;
        let left_keyboard = false;
        let high_contrast = false;
        let audio = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let mut engine = Engine {
            gpu,
            sprites,
            input,
            is_jumping,
            leftis_jumping,
            velocity_y,
            leftvelocity_y,
            gravity,
            score,
            single_player,
            sub_score,
            audio_cues,
            left_keyboard,
            high_contrast,
            audio,
        };

        let mut keyboard = Keyboard {
            left: VirtualKeyCode::Left,
            right: VirtualKeyCode::Right,
            up: VirtualKeyCode::Up,
            down: VirtualKeyCode::Down,
        };

        game.init(&mut engine).await;
        //let mut time = std::time::Instant::now();
        let mut p1_speed: f32 = 3.0;
        let mut p2_speed = 3.0; // This is the movement speed that will be affected when using powerups
        let mut frames = 0;

        event_loop.run(move |event, _, control_flow| {
            // By default, tell the windowing system that there's no more work to do
            // from the application's perspective.
            *control_flow = ControlFlow::Wait;
            // Depending on the event, we'll need to do different things.
            // There is some pretty fancy pattern matching going on here,
            // so think back to CSCI054.
            match event {
                Event::MainEventsCleared => {
                    // Print elapsed time
                    // println!("DT: {:?}", last_frame.elapsed());
                    // Reset last frame timer
                    //let time = std::time::Instant::now();
                }
                Event::WindowEvent {
                    // For example, "if it's a window event and the specific window event is that
                    // we have resized the window to a particular new size called `size`..."
                    event: WindowEvent::Resized(size),
                    // Ignoring the rest of the fields of Event::WindowEvent...
                    ..
                } => {
                    // Reconfigure the surface with the new size
                    engine.gpu.resize(size);
                    // On MacOS the window needs to be redrawn manually after resizing
                    window.request_redraw();
                }
                Event::WindowEvent {
                    // Note this deeply nested pattern match
                    event: WindowEvent::KeyboardInput { input: key_ev, .. },
                    ..
                } => {
                    engine.input.handle_key_event(key_ev);
                }

                Event::RedrawRequested(_) => {
                    //Multiple 60 by the amount of seconds
                    if frames > 420 {
                        //Slight Problem: If you get a powerup close to 6 seconds, the powerup will be removed immediatly

                        frames = 0;
                        p1_speed = 3.0;
                        p2_speed = 3.0;
                        //Refresh the sprites after they run out
                        engine.sprites.respawn_powerup(5);
                    }

                    // set high contrast
                    if engine
                        .input
                        .is_key_pressed(winit::event::VirtualKeyCode::Key1)
                        && options_shown
                    {
                        engine.high_contrast = !engine.high_contrast;
                        if engine.high_contrast {
                            engine.sprites.set_screen_size_sprite(15, 0, [32.0, 32.0])
                        } else {
                            let old_region = engine.sprites.get_sprites(15)[0].screen_region;
                            engine.sprites.zero_sprite(old_region, 15, 0);
                        }
                    }

                    // set audio cues
                    if engine
                        .input
                        .is_key_pressed(winit::event::VirtualKeyCode::Key2)
                        && options_shown
                    {
                        engine.audio_cues = !engine.audio_cues;
                        if engine.audio_cues {
                            engine.sprites.set_screen_size_sprite(15, 1, [32.0, 32.0])
                        } else {
                            let old_region = engine.sprites.get_sprites(15)[1].screen_region;
                            engine.sprites.zero_sprite(old_region, 15, 1);
                        }
                    }

                    // set left keyboard
                    if engine
                        .input
                        .is_key_pressed(winit::event::VirtualKeyCode::Key3)
                        && options_shown
                    {
                        engine.left_keyboard = !engine.left_keyboard;
                        if engine.left_keyboard {
                            engine.sprites.set_screen_size_sprite(15, 2, [32.0, 32.0]);
                            keyboard = Keyboard {
                                left: VirtualKeyCode::A,
                                right: VirtualKeyCode::D,
                                up: VirtualKeyCode::W,
                                down: VirtualKeyCode::S,
                            };
                        } else {
                            let old_region = engine.sprites.get_sprites(15)[2].screen_region;
                            engine.sprites.zero_sprite(old_region, 15, 2);
                            keyboard = Keyboard {
                                left: VirtualKeyCode::Left,
                                right: VirtualKeyCode::Right,
                                up: VirtualKeyCode::Up,
                                down: VirtualKeyCode::Down,
                            };
                        }
                    }

                    if engine
                        .input
                        .is_key_down(winit::event::VirtualKeyCode::Return)
                    {
                        // PLAY
                        if engine.sprites.get_sprites(10)[0].screen_region[1] == 400.0 {
                            engine.single_player = true;
                        }

                        if !engine.single_player {
                            if !options_shown {
                                engine
                                    .sprites
                                    .set_screen_size_sprite(14, 0, [1024.0, 768.0]);
                                let options = vec![
                                    engine.high_contrast,
                                    engine.audio_cues,
                                    engine.left_keyboard,
                                ];
                                for i in 0..3 {
                                    if options[i] {
                                        engine.sprites.set_screen_size_sprite(15, i, [32.0, 32.0]);
                                    }
                                }
                                options_shown = true;
                            }
                        } else {
                            let old_region = engine.sprites.get_sprites(6)[0].screen_region;
                            engine.sprites.zero_sprite(old_region, 6, 0);

                            //platformer
                            for i in 0..10 {
                                engine.sprites.zero_sprite(old_region, 7, i);
                            }
                            for i in 0..7 {
                                engine.sprites.zero_sprite(old_region, 8, i);
                            }
                            for i in 0..4 {
                                engine.sprites.zero_sprite(old_region, 9, i);
                            }

                            engine.sprites.zero_sprite(old_region, 10, 0);

                            if !already_removed_multi {
                                // zero out multiplayer stuff behind title screen
                                let old_region = engine.sprites.get_sprites(1)[0].screen_region;
                                engine.sprites.zero_sprite(old_region, 2, 0);
                                let old_region = engine.sprites.get_sprites(0)[0].sheet_region;
                                engine.sprites.update_sprite(
                                    [old_region[0], old_region[1], 0.75, old_region[3]],
                                    0,
                                );
                                let new_region = [150.0, 85.0, 64.0, 64.0];
                                engine.sprites.update_position(new_region, 3);
                                engine.sprites.update_sprite([0.0, 0.0, 0.0, 0.0], 5);
                                already_removed_multi = true;
                            }
                        }
                    }

                    if engine
                        .input
                        .is_key_down(winit::event::VirtualKeyCode::Escape)
                        && options_shown
                    {
                        let mut old_region = engine.sprites.get_sprites(14)[0].screen_region;
                        engine.sprites.zero_sprite(old_region, 14, 0);
                        for i in 0..3 {
                            old_region = engine.sprites.get_sprites(15)[i].screen_region;
                            engine.sprites.zero_sprite(old_region, 15, i);
                        }
                        options_shown = false;
                    }

                    if engine.input.is_key_down(keyboard.down) {
                        let old_position = engine.sprites.get_sprites(10)[0].screen_region;
                        engine
                            .sprites
                            .update_position([200.0, 400.0, old_position[2], old_position[3]], 10);
                    }
                    if engine.input.is_key_down(keyboard.up) {
                        let old_position = engine.sprites.get_sprites(10)[0].screen_region;
                        engine
                            .sprites
                            .update_position([200.0, 500.0, old_position[2], old_position[3]], 10);
                    }

                    if engine.input.is_key_down(keyboard.up) && !engine.is_jumping {
                        //engine.sprites.update_sprite_score([0.0, 0.54545456, 0.11111111, 0.09090909], 4, 0);
                        engine.is_jumping = true;
                        engine.velocity_y = 20.0; // This will be the upward force or the initial jump velocity. Adjust as needed.
                        let jump = StaticSoundData::from_file(
                            "scene2d/src/musiccontent/jump.mp3",
                            StaticSoundSettings::default(),
                        )
                        .unwrap();

                        engine.audio.play(jump);
                    }

                    if engine.leftis_jumping {
                        let mut the_move = true;
                        let old_region = engine.sprites.get_sprites(2)[0].screen_region;
                        engine.leftvelocity_y += engine.gravity; // Apply gravity to velocity
                        let new_y = old_region[1] + engine.leftvelocity_y;

                        let sprite_x = old_region[0];
                        let sprite_xw = old_region[0] + old_region[2];
                        let sprite_y = new_y;
                        let sprite_yh = new_y + old_region[3];

                        for i in 0..4 {
                            if (sprite_xw - 5.0 > engine.sprites.get_sprites(1)[i].screen_region[0])
                                && (sprite_x + 5.0
                                    < engine.sprites.get_sprites(1)[i].screen_region[0]
                                        + engine.sprites.get_sprites(1)[i].screen_region[2])
                                && (sprite_yh > engine.sprites.get_sprites(1)[i].screen_region[1])
                                && (sprite_y
                                    < engine.sprites.get_sprites(1)[i].screen_region[1]
                                        + engine.sprites.get_sprites(1)[i].screen_region[3])
                            {
                                the_move = false;
                            }
                        }

                        if !engine.single_player {
                            if ((sprite_xw - 5.0 > 150.0) && (sprite_x + 5.0 < (150.0 + 64.0)))
                                && (sprite_y < (130.0 + 64.0))
                            {
                                the_move = false;
                            }
                            if ((sprite_xw - 5.0 > 850.0) && (sprite_x + 5.0 < (850.0 + 64.0)))
                                && (sprite_y < (130.0 + 64.0))
                            {
                                the_move = false;
                            }
                        }

                        if the_move {
                            let new_region = [old_region[0], new_y, old_region[2], old_region[3]];
                            engine.sprites.update_position(new_region, 2);
                        }

                        // Check if the sprite has landed.
                        // Assuming 85.0 is ground level.
                        if new_y <= 85.0
                            || (sprite_yh > engine.sprites.get_sprites(1)[0].screen_region[1] - 3.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[0].screen_region[1] + 3.0)
                            || (sprite_yh > engine.sprites.get_sprites(1)[1].screen_region[1] - 1.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[1].screen_region[1] + 1.0)
                            || (sprite_yh > engine.sprites.get_sprites(1)[2].screen_region[1] - 1.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[2].screen_region[1] + 1.0)
                            || (sprite_yh > engine.sprites.get_sprites(1)[3].screen_region[1] - 1.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[3].screen_region[1] + 1.0)
                        {
                            engine.leftis_jumping = false;
                            engine.leftvelocity_y = 0.0;
                        }
                    }

                    if engine.is_jumping {
                        let mut the_move = true;
                        let old_region = engine.sprites.get_sprites(3)[0].screen_region;
                        engine.velocity_y += engine.gravity; // Apply gravity to velocity
                        let new_y = old_region[1] + engine.velocity_y;

                        let sprite_x = old_region[0];
                        let sprite_xw = old_region[0] + old_region[2];
                        let sprite_y = new_y;
                        let sprite_yh = new_y + old_region[3];

                        for i in 0..4 {
                            if (sprite_xw - 5.0 > engine.sprites.get_sprites(1)[i].screen_region[0])
                                && (sprite_x + 5.0
                                    < engine.sprites.get_sprites(1)[i].screen_region[0]
                                        + engine.sprites.get_sprites(1)[i].screen_region[2])
                                && (sprite_yh > engine.sprites.get_sprites(1)[i].screen_region[1])
                                && (sprite_y
                                    < engine.sprites.get_sprites(1)[i].screen_region[1]
                                        + engine.sprites.get_sprites(1)[i].screen_region[3])
                            {
                                the_move = false;
                            }
                        }
                        if !engine.single_player {
                            if ((sprite_xw - 5.0 > 150.0) && (sprite_x + 5.0 < (150.0 + 64.0)))
                                && (sprite_y < (130.0 + 64.0))
                            {
                                the_move = false;
                            }
                            if ((sprite_xw - 5.0 > 850.0) && (sprite_x + 5.0 < (850.0 + 64.0)))
                                && (sprite_y < (130.0 + 64.0))
                            {
                                the_move = false;
                            }
                        }
                        if the_move {
                            let new_region = [old_region[0], new_y, old_region[2], old_region[3]];
                            engine.sprites.update_position(new_region, 3);
                        }

                        // Check if the sprite has landed.a
                        // Assuming 85.0 is ground level.
                        if new_y <= 85.0
                            || (sprite_yh > engine.sprites.get_sprites(1)[0].screen_region[1] - 1.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[0].screen_region[1] + 1.0)
                            || (sprite_yh > engine.sprites.get_sprites(1)[1].screen_region[1] - 1.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[1].screen_region[1] + 1.0)
                            || (sprite_yh > engine.sprites.get_sprites(1)[2].screen_region[1] - 1.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[2].screen_region[1] + 1.0)
                            || (sprite_yh > engine.sprites.get_sprites(1)[3].screen_region[1] - 1.0
                                && sprite_yh
                                    < engine.sprites.get_sprites(1)[3].screen_region[1] + 1.0)
                        {
                            engine.is_jumping = false;
                            engine.velocity_y = 0.0;
                            let jump = StaticSoundData::from_file(
                                "scene2d/src/musiccontent/landed.mp3",
                                StaticSoundSettings::default().volume(2.5),
                            )
                            .unwrap();

                            engine.audio.play(jump);
                        }
                    }

                    if engine.input.is_key_down(keyboard.right) {
                        if engine.single_player {
                            engine.sub_score += 1;
                            if engine.sub_score == 5 {
                                engine.score += 1;
                                let score_digits = engine.sprites.update_score(engine.score);

                                engine.sprites.update_sprite_score(score_digits[0], 4, 6);
                                engine.sprites.update_sprite_score(score_digits[1], 4, 7);
                                engine.sprites.update_sprite_score(score_digits[2], 4, 8);

                                engine.sub_score = 0;
                            }
                        }

                        let new_sheet_pos = [0.0, 16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0];
                        engine.sprites.update_sprite(new_sheet_pos, 3);
                        if engine.single_player {
                            let mut the_move = true;
                            let new_region = engine.sprites.get_sprites(3)[0].screen_region;

                            let sprite_x = new_region[0];
                            let sprite_xw = new_region[0] + new_region[2];
                            let sprite_y = new_region[1];
                            let sprite_yh = new_region[1] + new_region[3];

                            for i in 0..4 {
                                if (sprite_xw - 5.0
                                    > engine.sprites.get_sprites(1)[i].screen_region[0])
                                    && (sprite_x + 5.0
                                        < engine.sprites.get_sprites(1)[i].screen_region[0]
                                            + engine.sprites.get_sprites(1)[i].screen_region[2])
                                    && (sprite_yh
                                        > engine.sprites.get_sprites(1)[i].screen_region[1])
                                    && (sprite_y
                                        < engine.sprites.get_sprites(1)[i].screen_region[1]
                                            + engine.sprites.get_sprites(1)[i].screen_region[3])
                                {
                                    the_move = false;
                                }
                            }

                            if the_move {
                                let old_sheet_pos = engine.sprites.get_sprites(0)[0].sheet_region;
                                engine.sprites.update_sprite(
                                    [
                                        old_sheet_pos[0] + (p2_speed / 3000.0),
                                        old_sheet_pos[1],
                                        0.75,
                                        old_sheet_pos[3],
                                    ],
                                    0,
                                );
                                if old_sheet_pos[0] + p2_speed / 3000.0 > 1.0 {
                                    engine.sprites.update_sprite(
                                        [-1.0, old_sheet_pos[1], 0.75, old_sheet_pos[3]],
                                        0,
                                    );
                                }
                                for i in 0..4 {
                                    let current = engine.sprites.get_sprites(1)[i].screen_region;
                                    let block = engine.sprites.get_sprite_mut(1, i);
                                    block.screen_region =
                                        [current[0] - p2_speed, current[1], current[2], current[3]];
                                    if current[0] - p2_speed < -64.0 {
                                        block.screen_region =
                                            [1084.0, current[1], current[2], current[3]];
                                    }
                                }
                            } else {
                                engine.is_jumping = false;
                            }
                        } else {
                            //Technically 0 Should always be the background
                            //2 should always be the sprite until i change it
                            let old_region = engine.sprites.get_sprites(3)[0].screen_region;
                            let new_region = [
                                old_region[0] + p2_speed,
                                old_region[1],
                                old_region[2],
                                old_region[3],
                            ];

                            let mut the_move = true;

                            let sprite_x = new_region[0];
                            let sprite_xw = new_region[0] + new_region[2];
                            let sprite_y = new_region[1];
                            let sprite_yh = new_region[1] + new_region[3];

                            for i in 0..4 {
                                if (sprite_xw - 5.0
                                    > engine.sprites.get_sprites(1)[i].screen_region[0])
                                    && (sprite_x + 5.0
                                        < engine.sprites.get_sprites(1)[i].screen_region[0]
                                            + engine.sprites.get_sprites(1)[i].screen_region[2])
                                    && (sprite_yh
                                        > engine.sprites.get_sprites(1)[i].screen_region[1])
                                    && (sprite_y
                                        < engine.sprites.get_sprites(1)[i].screen_region[1]
                                            + engine.sprites.get_sprites(1)[i].screen_region[3])
                                {
                                    the_move = false;
                                }
                            }
                            if !engine.single_player {
                                if (sprite_xw - 5.0 > 150.0 && sprite_x + 5.0 < 214.0)
                                    && sprite_y < 130.0 + 64.0
                                {
                                    the_move = false;
                                }

                                if (sprite_xw - 5.0 > 850.0 && sprite_x + 5.0 < 914.0)
                                    && sprite_y < 130.0 + 64.0
                                {
                                    the_move = false;
                                }
                            }
                            if new_region[0] + new_region[2] < 1030.0 && the_move {
                                engine.sprites.update_position(new_region, 3);
                            }
                        }
                    }
                    if engine.input.is_key_released(keyboard.right) {
                        let new_sheet_pos = [32.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0];
                        engine.sprites.update_sprite(new_sheet_pos, 3);
                    }
                    if engine.input.is_key_released(keyboard.left) {
                        let new_sheet_pos = [32.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0];
                        engine.sprites.update_sprite(new_sheet_pos, 3);
                    }
                    if engine.input.is_key_down(keyboard.left) {
                        let new_sheet_pos = [16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0];
                        engine.sprites.update_sprite(new_sheet_pos, 3);
                        if engine.single_player {
                            let mut the_move = true;
                            let new_region = engine.sprites.get_sprites(3)[0].screen_region;

                            let sprite_x = new_region[0];
                            let sprite_xw = new_region[0] + new_region[2];
                            let sprite_y = new_region[1];
                            let sprite_yh = new_region[1] + new_region[3];

                            for i in 0..4 {
                                if (sprite_xw - 5.0
                                    > engine.sprites.get_sprites(1)[i].screen_region[0])
                                    && (sprite_x + 5.0
                                        < engine.sprites.get_sprites(1)[i].screen_region[0]
                                            + engine.sprites.get_sprites(1)[i].screen_region[2])
                                    && (sprite_yh
                                        > engine.sprites.get_sprites(1)[i].screen_region[1])
                                    && (sprite_y
                                        < engine.sprites.get_sprites(1)[i].screen_region[1]
                                            + engine.sprites.get_sprites(1)[i].screen_region[3])
                                {
                                    the_move = false;
                                }
                            }

                            if the_move {
                                let old_sheet_pos = engine.sprites.get_sprites(0)[0].sheet_region;
                                engine.sprites.update_sprite(
                                    [
                                        old_sheet_pos[0] - (p2_speed / 3000.0),
                                        old_sheet_pos[1],
                                        0.75,
                                        old_sheet_pos[3],
                                    ],
                                    0,
                                );
                                if old_sheet_pos[0] + p2_speed / 3000.0 < -1.0 {
                                    engine.sprites.update_sprite(
                                        [1.0, old_sheet_pos[1], 0.75, old_sheet_pos[3]],
                                        0,
                                    );
                                }
                                for i in 0..4 {
                                    let current = engine.sprites.get_sprites(1)[i].screen_region;
                                    let block = engine.sprites.get_sprite_mut(1, i);
                                    block.screen_region =
                                        [current[0] + p2_speed, current[1], current[2], current[3]];
                                }
                            } else {
                                engine.leftis_jumping = false;
                            }

                            let old_sheet_pos = engine.sprites.get_sprites(0)[0].sheet_region;
                            engine.sprites.update_sprite(
                                [
                                    old_sheet_pos[0] - (p2_speed / 3000.0),
                                    old_sheet_pos[1],
                                    0.75,
                                    old_sheet_pos[3],
                                ],
                                0,
                            );

                            for i in 0..4 {
                                let current = engine.sprites.get_sprites(1)[i].screen_region;
                                let block = engine.sprites.get_sprite_mut(1, i);
                                block.screen_region =
                                    [current[0] + p2_speed, current[1], current[2], current[3]];
                            }
                        } else {
                            //Technically 0 Should always be the background
                            //2 should always be the sprite until i change it
                            let old_region = engine.sprites.get_sprites(3)[0].screen_region;
                            let new_region = [
                                old_region[0] - p2_speed,
                                old_region[1],
                                old_region[2],
                                old_region[3],
                            ];
                            //let new_sheet_pos = [16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0, 16.0 / 64.0];

                            let mut the_move = true;

                            let sprite_x = new_region[0];
                            let sprite_xw = new_region[0] + new_region[2];
                            let sprite_y = new_region[1];
                            let sprite_yh = new_region[1] + new_region[3];

                            for i in 0..4 {
                                if (sprite_xw - 5.0
                                    > engine.sprites.get_sprites(1)[i].screen_region[0])
                                    && (sprite_x + 5.0
                                        < engine.sprites.get_sprites(1)[i].screen_region[0]
                                            + engine.sprites.get_sprites(1)[i].screen_region[2])
                                    && (sprite_yh
                                        > engine.sprites.get_sprites(1)[i].screen_region[1])
                                    && (sprite_y
                                        < engine.sprites.get_sprites(1)[i].screen_region[1]
                                            + engine.sprites.get_sprites(1)[i].screen_region[3])
                                {
                                    the_move = false;
                                }
                            }

                            if !engine.single_player {
                                if (sprite_xw - 5.0 > 150.0 && sprite_x + 5.0 < 214.0)
                                    && sprite_y < 130.0 + 64.0
                                {
                                    the_move = false;
                                }

                                if (sprite_xw - 5.0 > 850.0 && sprite_x + 5.0 < 914.0)
                                    && sprite_y < 130.0 + 64.0
                                {
                                    the_move = false;
                                }
                            }

                            if new_region[0] > -5.0 && the_move {
                                engine.sprites.update_position(new_region, 3);
                            }
                        }
                    }

                    if engine.sprites.get_sprites(3)[0].screen_region[1] < 85.0 {
                        let old_region = engine.sprites.get_sprites(3)[0].screen_region;
                        let new_region = [old_region[0], 85.0, old_region[2], old_region[3]];
                        engine.sprites.update_position(new_region, 3);
                    }

                    if engine.sprites.get_sprites(2)[0].screen_region[1] < 85.0 {
                        let old_region = engine.sprites.get_sprites(2)[0].screen_region;
                        let new_region = [old_region[0], 85.0, old_region[2], old_region[3]];
                        engine.sprites.update_position(new_region, 2);
                    }
                    if engine.sprites.get_sprites(3)[0].screen_region[1] < 260.0
                        && !engine.is_jumping
                    {
                        let old_region = engine.sprites.get_sprites(3)[0].screen_region;

                        if engine.single_player {
                            let new_region = [old_region[0], 85.0, old_region[2], old_region[3]];
                            engine.sprites.update_position(new_region, 3);
                        } else {
                            let sprite_x = old_region[0];
                            let sprite_xw = old_region[0] + old_region[2];
                            if !(sprite_xw - 5.0 > 150.0 && sprite_x + 5.0 < 214.0)
                                && !(sprite_xw - 5.0 > 850.0 && sprite_x + 5.0 < 914.0)
                            {
                                let old_region = engine.sprites.get_sprites(3)[0].screen_region;

                                let new_region =
                                    [old_region[0], 85.0, old_region[2], old_region[3]];
                                engine.sprites.update_position(new_region, 3);
                            }
                        }
                    }
                    if engine.sprites.get_sprites(2)[0].screen_region[1] < 260.0
                        && !engine.leftis_jumping
                    {
                        let old_region = engine.sprites.get_sprites(2)[0].screen_region;

                        if engine.single_player {
                            let new_region = [old_region[0], 85.0, old_region[2], old_region[3]];
                            engine.sprites.update_position(new_region, 2);
                        } else {
                            let sprite_x = old_region[0];
                            let sprite_xw = old_region[0] + old_region[2];
                            if !(sprite_xw - 5.0 > 150.0 && sprite_x + 5.0 < 214.0)
                                && !(sprite_xw - 5.0 > 850.0 && sprite_x + 5.0 < 914.0)
                            {
                                let old_region = engine.sprites.get_sprites(2)[0].screen_region;

                                let new_region =
                                    [old_region[0], 85.0, old_region[2], old_region[3]];
                                engine.sprites.update_position(new_region, 2);
                            }
                        }
                    }

                    // engine.sprites.platform_move();

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        1,
                        0..(engine.sprites.get_sprites(1).len()),
                    );

                    //This refreshes the sprite player group to update the position of both sprites
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        2,
                        0..(engine.sprites.get_sprites(2).len()),
                    );

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        3,
                        0..(engine.sprites.get_sprites(3).len()),
                    );
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        4,
                        0..(engine.sprites.get_sprites(4).len()),
                    );

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        5,
                        0..(engine.sprites.get_sprites(5).len()),
                    );

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        6,
                        0..(engine.sprites.get_sprites(6).len()),
                    );
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        7,
                        0..(engine.sprites.get_sprites(7).len()),
                    );
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        8,
                        0..(engine.sprites.get_sprites(8).len()),
                    );
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        9,
                        0..(engine.sprites.get_sprites(9).len()),
                    );
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        10,
                        0..(engine.sprites.get_sprites(10).len()),
                    );

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        11,
                        0..(engine.sprites.get_sprites(11).len()),
                    );

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        12,
                        0..(engine.sprites.get_sprites(12).len()),
                    );
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        13,
                        0..(engine.sprites.get_sprites(13).len()),
                    );
                    engine.sprites.refresh_sprites(&engine.gpu, 14, 0..1);
                    engine.sprites.refresh_sprites(&engine.gpu, 15, 0..3);
                    //Checking if the p1 sprite, collides with group 5, which are only powerups (Speed Up Powerups)
                    if engine.sprites.check_collisions(2, 5) {
                        //If so we update their speed (How many pixels they travel across the screen)
                        p1_speed = 7.0;
                    }

                    //Check Player2
                    if engine.sprites.check_collisions(3, 5) {
                        p2_speed = 7.0
                    }

                    game.update(&mut engine);
                    engine.input.next_frame();
                    if engine.sprites.player_collision() {
                        if engine.score > 100 {
                            //Queue Winning Screen
                            println!("Uhh some player one Shrug");
                            *control_flow = ControlFlow::Exit;
                        }
                        engine.score += 5;
                        let score_digits = engine.sprites.update_score(engine.score);

                        engine.sprites.update_sprite_score(score_digits[0], 4, 6);
                        engine.sprites.update_sprite_score(score_digits[1], 4, 7);
                        engine.sprites.update_sprite_score(score_digits[2], 4, 8);
                    }
                    frames += 1;

                    // If the window system is telling us to redraw, let's get our next swapchain image
                    let frame = engine
                        .gpu
                        .surface
                        .get_current_texture()
                        .expect("Failed to acquire next swap chain texture");
                    // And set up a texture view onto it, since the GPU needs a way to interpret those
                    // image bytes for writing.
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    // From the queue we obtain a command encoder that lets us issue GPU commands
                    let mut encoder = engine
                        .gpu
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        // Now we begin a render pass.  The descriptor tells WGPU that
                        // we want to draw onto our swapchain texture view (that's where the colors will go)
                        // and that there's no depth buffer or stencil buffer.
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });
                        engine.sprites.render(&mut rpass);
                    }

                    // Once the commands have been scheduled, we send them over to the GPU via the queue.
                    engine.gpu.queue.submit(Some(encoder.finish()));
                    // Then we wait for the commands to finish and tell the windowing system to
                    // present the swapchain image.
                    frame.present();

                    // (3)
                    // And we have to tell the window to redraw!
                    window.request_redraw(); // Creates a loop and procedds to redraw the window
                }
                // If we're supposed to close the window, tell the event loop we're all done
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                // Ignore every other event for now.
                _ => {}
            }
        });
    }
    pub fn load_texture(
        &self,
        path: impl AsRef<std::path::Path>,
        label: Option<&str>,
    ) -> Result<(wgpu::Texture, image::RgbaImage), image::ImageError> {
        self.gpu.load_texture(path.as_ref(), label)
    }
}
