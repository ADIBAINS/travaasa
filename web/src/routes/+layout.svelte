<script lang="ts">
    import { onMount } from 'svelte';
    import type { LayoutProps } from './$types';
    import './app.css';

    let { data, children }: LayoutProps = $props();
    let dark = $state(false);

    onMount(() => {
        const theme = localStorage.getItem('travaasa-theme');
        dark = theme ? theme === 'dark' : matchMedia('(prefers-color-scheme: dark)').matches;
        document.documentElement.dataset.theme = dark ? 'dark' : 'light';
    });

    function toggleTheme() {
        dark = !dark;
        const theme = dark ? 'dark' : 'light';
        document.documentElement.dataset.theme = theme;
        localStorage.setItem('travaasa-theme', theme);
    }
</script>

<svelte:head>
    <title>Travaasa</title>
    <meta name="description" content="Explore and save stays for your next trip." />
</svelte:head>

<a class="skip-link" href="#main">Skip to main content</a>
<header class="site-header">
    <nav class="frame nav" aria-label="Main navigation">
        <a class="brand" href="/listings">Travaasa</a>
        <div class="nav-links">
            {#if data.user}
                <a href="/listings/saved">Saved stays</a>
                <a href="/listings/new">Host a stay</a>
                <form method="post" action="/logout"><button class="text-button">Sign out</button></form>
            {:else}
                <a href="/login">Sign in</a>
                <a href="/signup">Sign up</a>
            {/if}
            <button class="text-button" onclick={toggleTheme} aria-pressed={dark}>Dark mode</button>
        </div>
    </nav>
</header>
<main id="main" class="frame main">
    {@render children()}
</main>
<footer><div class="frame">Travaasa</div></footer>
