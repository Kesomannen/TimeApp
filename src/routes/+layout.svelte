<script>
	import { SvelteUIProvider } from '@svelteuidev/core';
</script>

<SvelteUIProvider themeObserver="dark" withNormalizeCSS withGlobalStyles>
    <slot />
</SvelteUIProvider>

<style>
    :global(html) {
        padding: 1.5rem;
    }

    :global(body) {
        font-family: 'Inter', sans-serif;
    }
</style>