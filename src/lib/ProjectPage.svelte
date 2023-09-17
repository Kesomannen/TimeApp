<script lang="ts">
    import { listen } from '@tauri-apps/api/event'
    import type { Event } from '@tauri-apps/api/event'
	import ProjectTimer from './ProjectTimer.svelte';

	import { Loader, Center, Divider } from '@svelteuidev/core';
	import { format_time } from './utils';
	import type { UpdatePayload, Project } from './types';
	import { onMount } from 'svelte';
    
    let projects: {name: string, project: Project}[] = []
    let loaded = false

    onMount(() => {
        listen('update', (event: Event<UpdatePayload>) => {
            let payload = event.payload
            projects = []

            for (let i = 0; i < payload.project_names.length; i++) {
                projects.push({
                    name: payload.project_names[i],
                    project: payload.projects[i]
                })
            }
            
            projects.sort((a, b) => {
                if (a.project.open && !b.project.open) return -1
                if (!a.project.open && b.project.open) return 1
                return b.project.time.secs - a.project.time.secs
            })

            loaded = true
        });
    });

    $: total_time = projects.reduce((acc, p) => acc + p.project.time.secs, 0)
</script>

{#if loaded}
    {#if projects.length === 0}
        No projects yet, open one to get started!
    {/if}
    
    <div id="project-list">
        {#each projects as { name, project }}
            <ProjectTimer name={name} project={project} />
        {/each}
    </div>

    <Divider />

    <div id="total">Total time spent developing: {format_time(total_time)}</div>
{:else}
    <Center>
        <Loader size='xl' />
    </Center>
{/if}

<style>
    #project-list {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }
</style>