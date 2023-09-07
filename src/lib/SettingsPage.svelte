<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import { Checkbox } from '@svelteuidev/core';

    let auto_start: boolean;

    onMount(() => {
        invoke<boolean>('get_auto_start')
            .then((res) => auto_start = res);
    });

    $: invoke('set_auto_start', { enabled: auto_start });
</script>

<div class="setting">
    Auto Start
    <Checkbox bind:checked={auto_start} />
</div>

<style>
    * {
        margin: 0.5rem 0;
    }

    .setting {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 1rem;
    }
</style>