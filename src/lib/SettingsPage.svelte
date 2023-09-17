<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import { NativeSelect, Checkbox } from '@svelteuidev/core';

    type Engine = 'Unity' | 'Godot';

    onMount(async () => {
        engine = await get('engine');
        autoStart = await get('auto_start');
    });

    let engine: Engine;
    let autoStart: boolean;

    $: set('engine', engine);
    $: set('auto_start', autoStart);

    function set<T>(key: string, value: T): Promise<void> {
        return invoke('set_key', { key, value });
    }

    function get<T>(key: string): Promise<T> {
        return invoke('get_key', { key });
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
        <div class="label">Auto start</div>
        <Checkbox bind:checked={autoStart} />
    </div>
</div>

<style>
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