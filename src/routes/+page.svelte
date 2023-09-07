<script>
	import { Tabs } from '@svelteuidev/core';
    import ProjectPage from '../lib/ProjectPage.svelte';
    import { Gear, ViewHorizontal } from 'radix-icons-svelte';
	import SettingsPage from '$lib/SettingsPage.svelte';
</script>

<Tabs>
    <Tabs.Tab label="Projects" tabKey="first" icon={ViewHorizontal}><ProjectPage /></Tabs.Tab>
    <Tabs.Tab label="Settings" tabKey="second" icon={Gear}><SettingsPage /></Tabs.Tab>
</Tabs>