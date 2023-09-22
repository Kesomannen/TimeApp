<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import { NativeSelect, Checkbox, colorScheme } from '@svelteuidev/core';
	import type { Theme, Engine } from './types';

    onMount(async () => {
        engine = await get('engine');
        autoStart = await get('auto_start');
        theme = await get('appearance');
    });

    let engine: Engine;
    let autoStart: boolean;
    let theme: Theme;

    $: set('engine', engine);
    $: set('auto_start', autoStart ? 'true' : 'false');
    $: set('theme', theme);
    $: colorScheme.set(theme === 'Dark' ? 'dark' : 'light');

    function set<T>(key: string, value: T): Promise<void> {
        return invoke('set_key', { key: key, value: value });
    }

    function get<T>(key: string): Promise<T> {
        return invoke('get_key', { key: key });
    }
</script>

<div id="list">
    <div class="setting">
        <div class="label">Engine</div>
        <NativeSelect 
            data={['Unity', 'Godot']}
            placeholder="Select engine"
            variant="filled"
            bind:value={engine}
        />
    </div>
    <div class="setting">
        <div class="label">Theme</div>
        <NativeSelect 
            data={['Light', 'Dark']}
            placeholder="Select theme"
            variant="filled"
            bind:value={theme}
        />
    </div>
    <div class="setting">
        <div class="label">Auto Start</div>
        <Checkbox bind:checked={autoStart} />
    </div>
</div>

<style>
    .label {
        font-weight: 500;
    }

    #list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .setting {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
</style>