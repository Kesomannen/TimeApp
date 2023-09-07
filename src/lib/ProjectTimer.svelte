<script lang="ts">
	import { Button, Text } from '@svelteuidev/core';
    import { invoke } from '@tauri-apps/api/tauri';
    import { format_time } from './utils';
    import type { Project } from './types';

    export let name: string;
    export let project: Project;
</script>

<div class="container">
    {#if project.open}
        <Text color="lime" weight="bold">
            OPEN
        </Text>
    {/if}

    <Text>{project.display_name}</Text>

    <Text weight="bold">
        {format_time(project.time.secs)}
    </Text>

    <Button variant="subtle" color="red" on:click={() => invoke('remove_project', { name: name })}>
        {project.open ? 'Reset' : 'Remove'}
    </Button>
</div>

<style>
    .container {
        display: flex;
        align-items: center;
        gap: 1rem;
    }
</style>