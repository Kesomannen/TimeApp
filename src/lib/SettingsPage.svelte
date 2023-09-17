<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import { NativeSelect } from '@svelteuidev/core';

    type Engine = 'Unity' | 'Godot';

    onMount(async () => {
        engine = await invoke('get_key', { key: 'engine' });
    });

    let engine: Engine;

    $: invoke('set_key', { key: 'engine', value: engine });
</script>

<div class="setting">
    <div class="label">Engine</div>
    <NativeSelect 
        data={['Unity', 'Godot']}
        placeholder="Select engine"
        variant="filled"
        bind:value={engine}
    />
</div>

<style>
    .setting {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
</style>