#pragma once
#include <functional>
#include <optional>
#include <uhid/ps5.hpp>
#include <uhid/uhid.hpp>

namespace inputtino {
struct PS5JoypadState {
  std::shared_ptr<uhid::Device> dev;
  /**
   * This will be the MAC address of the device
   *
   * IMPORTANT: this needs to be unique for each virtual device,
   * otherwise the kernel driver will return an error:
   * "Duplicate device found for MAC address XX:XX:XX:XX"
   *
   * We also use this information internally to unique match a device with the
   * /dev/input/devXX files; see get_nodes()
   */
  unsigned char mac_address[6] = {
      0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF
  };

  uhid::dualsense_input_report_usb current_state;
  uint8_t touch_points_ids[2] = {0};

  std::optional<std::function<void(int, int)>> on_rumble = std::nullopt;
  std::optional<std::function<void(int, int, int)>> on_led = std::nullopt;
};
} // namespace inputtino