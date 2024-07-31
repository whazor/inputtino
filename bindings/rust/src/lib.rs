#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod common;
pub mod mouse;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_inputtino_mouse() {
        let device = crate::common::InputtinoDeviceDefinition::new("Rusty Mouse", 0, 0, 0, "Rusty Mouse Phys", "Rusty Mouse Uniq");
        let mouse = crate::mouse::InputtinoMouse::new(&device).unwrap();
        let nodes = mouse.get_nodes().unwrap();
        assert_eq!(nodes.len(), 2);

        // Check that the nodes start with /dev/input/event
        assert!(nodes[0].starts_with("/dev/input/event"));
        assert!(nodes[1].starts_with("/dev/input/event"));

        // TODO: test the followings with libinput
        mouse.move_rel(10, 10);
        mouse.move_abs(100, 100, 1920, 1080);
        mouse.press_button(INPUTTINO_MOUSE_BUTTON::LEFT);
        mouse.release_button(INPUTTINO_MOUSE_BUTTON::LEFT);
        mouse.scroll_vertical(100);
        mouse.scroll_horizontal(100);
    }
}
