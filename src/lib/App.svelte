<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import type { Event } from '@tauri-apps/api/event'
	import Project from './Project.svelte';

	import { Loader, Center, Divider } from '@svelteuidev/core';
	import { format_time } from './utils';

    type Project = {
        name: string
        open: boolean
        time: {
            secs: number
            nanos: number
        }
    }

    type UpdatePayload = {
        status: string
        projects: Project[]
    }

    let projects: Project[] = []
    let loaded = false

    listen('update', (event: Event<UpdatePayload>) => {
        projects = event.payload.projects.toSorted((a, b) => {
            if (a.open && !b.open) return -1
            if (!a.open && b.open) return 1
            return b.time.secs - a.time.secs
        })

        loaded = true
    })

    $: total_time = projects.reduce((acc, project) => acc + project.time.secs, 0)
</script>

{#if loaded}
    <h1>Projects</h1>

    {#if projects.length === 0}
        <strong>No projects yet, open one in Unity to get started!</strong>
    {/if}
    
    <div class="project-list">
        {#each projects as project}
        <Project name={project.name} secs={project.time.secs} open={project.open} />
        {/each}
    </div>

    <Divider />

    <div class="total">Total time spent developing: {format_time(total_time)}</div>
{:else}
    <Center>
        <Loader size='xl' />
    </Center>
{/if}

<style>
    .project-list {
        display: flex;
        flex-direction: column;
    }

    .total {
        padding-top: 0.33rem;
    }
</style>