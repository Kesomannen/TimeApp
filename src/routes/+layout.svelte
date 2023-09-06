<script>
	import { SvelteUIProvider, AppShell } from '@svelteuidev/core';
</script>

<SvelteUIProvider themeObserver="dark" withNormalizeCSS withGlobalStyles>
    <slot />
</SvelteUIProvider>

<style>
    :global(html) {
        padding: 2rem;
    }

    :global(body) {
        font-family: 'Inter', sans-serif;
    }
</style>