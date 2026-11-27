<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/state';
    import Icon from '#lib/Icon.svelte';
    import type { LayoutProps } from './$types';
    import './app.css';

    let { data, children }: LayoutProps = $props();
    let dark = $state(false);

    onMount(() => {
        try {
            const theme = localStorage.getItem('travaasa-theme');
            dark = theme ? theme === 'dark' : matchMedia('(prefers-color-scheme: dark)').matches;
        } catch {
            dark = matchMedia('(prefers-color-scheme: dark)').matches;
        }
        document.documentElement.dataset.theme = dark ? 'dark' : 'light';
    });

    function toggleTheme() {
        dark = !dark;
        const theme = dark ? 'dark' : 'light';
        document.documentElement.dataset.theme = theme;
        try { localStorage.setItem('travaasa-theme', theme); } catch { /* Keep the theme for this page. */ }
    }
</script>

<svelte:head>
    <title>Travaasa</title>
    <meta name="description" content="Explore, save, and compare stays for your next trip." />
</svelte:head>

<a class="skip-link" href="#main">Skip to main content</a>
<header class="site-header">
    <nav class="frame site-nav" aria-label="Main navigation">
        <a class="brand" href="/listings"><Icon name="compass" size={30} /> Travaasa<span>.</span></a>
        <div class="site-links">
            <a href="/listings" aria-current={page.url.pathname === '/listings' ? 'page' : undefined}>Explore</a>
            <a href="/listings/saved" aria-current={page.url.pathname === '/listings/saved' ? 'page' : undefined}>Saved stays</a>
            {#if data.user}
                <a href="/listings/mine" aria-current={page.url.pathname === '/listings/mine' ? 'page' : undefined}>Your listings</a>
            {/if}
        </div>
        <div class="account-links">
            <button class="icon-button theme-button" onclick={toggleTheme} aria-label="Dark mode" aria-pressed={dark}>
                <Icon name={dark ? 'sun' : 'moon'} size={18} />
            </button>
            {#if data.user}
                <form method="post" action="/logout"><button class="text-button sign-out">Sign out</button></form>
            {:else}
                <a class="sign-in" href="/login">Sign in</a>
                <a class="button primary header-signup" href="/signup">Sign up</a>
            {/if}
        </div>
    </nav>
</header>
<main id="main" class="frame main">
    {@render children()}
</main>
<footer class="site-footer">
    <div class="frame footer-content">
        <a class="brand" href="/listings"><Icon name="compass" size={24} /> Travaasa<span>.</span></a>
        <nav aria-label="Footer">
            <a href="/listings">Explore stays</a>
            <a href="/listings/compare">Compare stays</a>
            <a href="/listings/new">Host a stay</a>
        </nav>
    </div>
</footer>
