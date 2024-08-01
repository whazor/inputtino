use input::{Event, Libinput};
use input::event::{DeviceEvent};
use input::event::keyboard::KeyboardEventTrait;
use inputtino_rs::{common::InputtinoDeviceDefinition,
                   keyboard::InputtinoKeyboard};
mod common;
use crate::common::{NixInterface, SyncEvent};


#[test]
fn test_inputtino_keyboard() {
    let device = InputtinoDeviceDefinition::new("Rusty Keyboard", 0xAB, 0xCD, 0xEF, "Rusty Keyboard Phys", "Rusty Keyboard Uniq");
    let keyboard = InputtinoKeyboard::new(&device).unwrap();
    let nodes = keyboard.get_nodes().unwrap();
    {
        assert_eq!(nodes.len(), 1);

        // Check that the nodes start with /dev/input/event
        assert!(nodes[0].starts_with("/dev/input/event"));
    }

    let mut input = Libinput::new_from_path(NixInterface);
    let kb_dev = input.path_add_device(nodes[0].as_str()).expect("to get the device");

    {
        assert_eq!(kb_dev.name(), "Rusty Keyboard");
        assert_eq!(kb_dev.id_vendor(), 0xAB);
        assert_eq!(kb_dev.id_product(), 0xCD);
        for event in &mut input {
            assert!(matches!(event, Event::Device(DeviceEvent::Added(_))));
        }
    }

    { // Test keyboard key press
        keyboard.press_key(0x41); // KEY_A

        let ev = input.wait_next_event().unwrap();
        assert!(matches!(ev, Event::Keyboard(_)));
        match ev {
            Event::Keyboard(ev) => {
                assert_eq!(ev.key(), 30); // KEY_A
                assert_eq!(ev.key_state(), input::event::keyboard::KeyState::Pressed);
            }
            _ => unreachable!(),
        }
    }

    { // Test keyboard key release
        keyboard.release_key(0x41);

        let ev = input.wait_next_event().unwrap();
        assert!(matches!(ev, Event::Keyboard(_)));
        match ev {
            Event::Keyboard(ev) => {
                assert_eq!(ev.key(), 30); // KEY_A
                assert_eq!(ev.key_state(), input::event::keyboard::KeyState::Released);
            }
            _ => unreachable!(),
        }
    }
}
