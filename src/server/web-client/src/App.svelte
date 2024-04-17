<script lang="ts">
    // import svelteLogo from './assets/svelte.svg'
    // import viteLogo from '/vite.svg'
    import Devices from './lib/Devices.svelte'
    import AddDevice from "./lib/AddDevice.svelte";
    import RemoveDevice from "./lib/RemoveDevice.svelte";
    import FetchRequest from "./lib/FetchRequest.svelte";
    import {selected_device} from "./lib/stores";
    import Mouse from "./lib/Mouse.svelte";
    import {Toolbar, ToolbarGroup, DarkMode, Toast} from 'flowbite-svelte';
    import {FireOutline} from "flowbite-svelte-icons";
    import Keyboard from "./lib/Keyboard.svelte";
</script>

<main>
    <Toolbar class="bg-slate-200 dark:bg-gray-700">
        <AddDevice/>
        <RemoveDevice/>
        <Devices/>

        <DarkMode slot="end"/>
    </Toolbar>

    {#if $selected_device?.type === "MOUSE"}
        <Mouse/>
    {/if}

    {#if $selected_device?.type === "KEYBOARD"}
        <Keyboard/>
    {/if}

    <FetchRequest/>
</main>

<style>
</style>
