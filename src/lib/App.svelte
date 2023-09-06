<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import type { Event } from '@tauri-apps/api/event'
	import ProjectTimer from './ProjectTimer.svelte';

	import { Loader, Center, Divider } from '@svelteuidev/core';
	import { format_time } from './utils';
	import type { UpdatePayload, Project } from './types';
    
    let project_names: string[] = []
    let projects: Project[] = []
    let loaded = false

    listen('update', (event: Event<UpdatePayload>) => {
        project_names = event.payload.project_names
        projects = event.payload.projects
        
        projects.sort((a, b) => {
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
        {#each projects as project, i}
            <ProjectTimer name={project_names[i]} project={project} />
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