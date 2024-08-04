use inputtino_rs::common::InputtinoDeviceDefinition;
use inputtino_rs::joypad_xbox::{InputtinoXOneJoypad, INPUTTINO_JOYPAD_BTN, INPUTTINO_JOYPAD_STICK_POSITION};

#[test]
fn test_xbox_joypad() {
    let device = InputtinoDeviceDefinition::new("Rusty XOne controller", 0x045e, 0x02dd, 0x0100, "00:11:22:33:44", "00:11:22:33:44");
    let mut joypad = InputtinoXOneJoypad::new(&device).unwrap();

    let nodes = joypad.get_nodes().unwrap();
    {
        assert_eq!(nodes.len(), 2);
        assert!(nodes[0].starts_with("/dev/input/event"));
        assert!(nodes[1].starts_with("/dev/input/js"));
    }

    let sdl = sdl2::init().unwrap();
    let joystick_subsystem = sdl.game_controller().unwrap();
    let mut sdl_js = joystick_subsystem.open(0).unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::JoyDeviceAdded { which, .. } => {
                assert_eq!(which, 0);
            }
            sdl2::event::Event::ControllerDeviceAdded { which, .. } => {
                assert_eq!(which, 0);
            }
            _ => panic!("Unexpected event : {:?}", event),
        }
    }

    assert_eq!(sdl_js.name(), "Xbox One Controller");
    assert!(sdl_js.has_rumble());

    {
        joypad.set_pressed(INPUTTINO_JOYPAD_BTN::A as i32);
        for event in event_pump.wait_timeout_iter(50) {
            match event {
                sdl2::event::Event::ControllerButtonDown { button, .. } => {
                    assert_eq!(button, sdl2::controller::Button::A);
                }
                sdl2::event::Event::JoyButtonDown { button_idx, .. } => {
                    assert_eq!(button_idx, sdl2::controller::Button::A as u8);
                    break;
                }
                _ => panic!("Unexpected event : {:?}", event),
            }
        }
    }

    {
        joypad.set_triggers(0, 0);
        for event in event_pump.wait_timeout_iter(50) {
            match event {
                sdl2::event::Event::ControllerAxisMotion { axis, value, .. } => {
                    assert_eq!(axis, sdl2::controller::Axis::TriggerLeft);
                    assert_eq!(value, 0);
                }
                sdl2::event::Event::JoyAxisMotion { axis_idx, value, .. } => {
                    assert_eq!(axis_idx, sdl2::controller::Axis::TriggerLeft as u8);
                    assert_eq!(value, 0);
                    break;
                }
                _ => panic!("Unexpected event : {:?}", event),
            }
        }
    }

    {
        joypad.set_stick(INPUTTINO_JOYPAD_STICK_POSITION::LS, 0, 0);
        for event in event_pump.wait_timeout_iter(50) {
            match event {
                sdl2::event::Event::ControllerAxisMotion { axis, value, .. } => {
                    assert_eq!(axis, sdl2::controller::Axis::LeftX);
                    assert_eq!(value, 0);
                }
                sdl2::event::Event::JoyAxisMotion { axis_idx, value, .. } => {
                    assert_eq!(axis_idx, sdl2::controller::Axis::LeftX as u8);
                    assert_eq!(value, 0);
                    break;
                }
                _ => panic!("Unexpected event : {:?}", event),
            }
        }
    }

    {
        joypad.set_stick(INPUTTINO_JOYPAD_STICK_POSITION::RS, 0, 0);
        for event in event_pump.wait_timeout_iter(50) {
            match event {
                sdl2::event::Event::ControllerAxisMotion { axis, value, .. } => {
                    assert_eq!(axis, sdl2::controller::Axis::RightX);
                    assert_eq!(value, 0);
                }
                sdl2::event::Event::JoyAxisMotion { axis_idx, value, .. } => {
                    assert_eq!(axis_idx, sdl2::controller::Axis::RightX as u8);
                    assert_eq!(value, 0);
                    break;
                }
                _ => panic!("Unexpected event : {:?}", event),
            }
        }
    }

    {
        joypad.set_on_rumble(move |left, right| {
            assert_eq!(left, 100);
            assert_eq!(right, 200);
        });
        let res = sdl_js.set_rumble(100, 200, 150);
        assert!(res.is_ok());
        std::thread::sleep(std::time::Duration::from_millis(25));
        joypad.set_on_rumble(move |left, right| {
            assert_eq!(left, 0);
            assert_eq!(right, 0);
        });
        std::thread::sleep(std::time::Duration::from_millis(125));
    }
}
