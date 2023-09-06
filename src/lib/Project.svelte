<script lang="ts">
	import { Button, Text } from '@svelteuidev/core';
    import { format_time } from './utils';
    import { invoke } from '@tauri-apps/api/tauri';

    export let name: string;
    export let secs: number;
    export let open: boolean;
</script>

<div class="container">
    {#if open}
        <Text color="lime" weight="bold" size="">
            OPEN
        </Text>
    {/if}

    {name}

    <strong>{format_time(secs)}</strong>

    <Button variant="subtle" color="red" on:click={() => invoke('remove_project', { name: name })}>
        {open ? 'Reset' : 'Remove'}
    </Button>
</div>

<style>
    .container {
        display: flex;
        align-items: center;
        gap: 1rem;
    }
</style>