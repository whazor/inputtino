<script lang="ts">

    import {request, selected_device} from "./stores";
    import {onDestroy, onMount} from "svelte";

    function keydown(e: KeyboardEvent) {
        if ($selected_device?.type === "KEYBOARD") {
            request("/devices/keyboard/" + $selected_device?.device_id + "/press", "POST", {
                "key": 65 // hardcoded a
                // TODO: e.key to win32 keycode
                //   see: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
            });

            // TODO: check for shift, ctrl, alt, etc..
        }
    }

    function keyup(e: KeyboardEvent) {
        if ($selected_device?.type === "KEYBOARD") {
            request("/devices/keyboard/" + $selected_device?.device_id + "/release", "POST", {
                "key": 65 // hardcoded a
            })
        }
    }

</script>

<input on:keydown={keydown}
       on:keyup={keyup}>