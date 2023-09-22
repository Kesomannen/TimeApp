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

    <span id="open" class={project.open ? "shown" : "hidden"}>
        OPEN
    </span>

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
        font-weight: 500;
    }
    
    .shown {
        opacity: 1;
        color: #12b886;
    }

    .hidden {
        opacity: 0;
        color: #12b886;
    }

    .shown, .hidden {
        transition: opacity 0.33s ease-out;
    }

    .shown {
        transition-delay: 0.5s;
    }

    .hidden {
        transition-delay: 0s;
    }
</style>