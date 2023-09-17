<script lang="ts">
	import { Button, ActionIcon } from '@svelteuidev/core';
    import { invoke } from '@tauri-apps/api/tauri';
    import { format_time } from './utils';
    import type { Project } from './types';
    import { Trash } from 'radix-icons-svelte';

    export let name: string;
    export let project: Project;

    function remove_project() {
        invoke('remove_project', { name });
    }
</script>

<div id="container">
    <span id="name">{project.display_name}</span>

    {format_time(project.time.secs)}

    <div id="fill"></div>

    {#if project.open}
        <span id="open">
            OPEN
        </span>
    {/if}

    <ActionIcon on:click={() => remove_project()} color="red">
        <Trash />
    </ActionIcon>
</div>

<style>
    #container {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    #fill {
        flex: 1;
    }

    #name {
        font-weight: 500;
    }

    #open {
        color: #12b886;
        font-weight: 500;
    }
</style>