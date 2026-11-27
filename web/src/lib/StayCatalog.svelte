<script lang="ts">
    import { enhance, type SubmitFunction } from '$app/forms';
    import { onMount } from 'svelte';
    import { categories, nightlyPrice, type Catalog } from './stays.js';
    import { comparisonIds, comparisonStorageKey, comparisonUrl, maxComparison } from './compare.js';
    import Icon from './Icon.svelte';
    import StayImage from './StayImage.svelte';

    let { catalog, filters, base, mode, message, signedIn, savedIds, actionMessage, savedChange }: {
        catalog: Catalog;
        filters: Record<string, string>;
        base: string;
        mode: 'explore' | 'saved' | 'mine';
        message: string;
        signedIn: boolean;
        savedIds: string[];
        actionMessage?: string;
        savedChange?: string;
    } = $props();

    let showTax = $state(false);
    let selected = $state<string[]>([]);
    let announced = $state('');
    let pending = $state<string[]>([]);
    let ready = $state(false);
    let explore = $derived(mode === 'explore');
    let heading = $derived(mode === 'mine' ? 'Your listings' : mode === 'saved' ? 'Saved stays' : 'Explore stays');
    let filtering = $derived(Boolean(filters.q || filters.category || filters.min || filters.max || (filters.sort && filters.sort !== 'newest')));

    onMount(() => {
        try { selected = comparisonIds(JSON.parse(localStorage.getItem(comparisonStorageKey) ?? '[]')); }
        catch { selected = []; }
        ready = true;
    });

    function setComparison(ids: string[], message: string) {
        selected = ids;
        announced = message;
        try { localStorage.setItem(comparisonStorageKey, JSON.stringify(ids)); }
        catch { /* Use the selection for this page when browser storage is unavailable. */ }
    }

    function toggleComparison(id: string) {
        if (selected.includes(id)) {
            setComparison(selected.filter((value) => value !== id), 'Stay removed from comparison.');
        } else if (selected.length < maxComparison) {
            setComparison([...selected, id], 'Stay added to comparison.');
        } else {
            announced = 'Clear or remove a stay before adding another.';
        }
    }

    function catalogLink(changes: Record<string, string> = {}, action = false) {
        const query = new URLSearchParams(Object.entries({ ...filters, ...changes }).filter(([, value]) => value));
        if (action) query.set('/toggleSaved', '');
        return `${base}${query.size ? `?${query}` : ''}${action ? '' : '#stays'}`;
    }

    const submitSave: SubmitFunction = ({ formData }) => {
        const id = String(formData.get('id'));
        pending = [...pending, id];
        return async ({ update }) => {
            try { await update({ reset: false }); }
            finally { pending = pending.filter((value) => value !== id); }
        };
    };
</script>

<svelte:head><title>{heading} | Travaasa</title></svelte:head>

{#if explore}
    <section class="discovery-hero" aria-labelledby="hero-title">
        <img class="discovery-cover" src="/images/alpine-getaway.webp" alt="" fetchpriority="high" />
        <div class="hero-inner">
            <div class="hero-copy">
                <h1 id="hero-title">Find your kind<br />of getaway.</h1>
                <p>From tucked-away cabins to city hideouts, discover a place worth the trip.</p>
            </div>
            <form class="hero-search" method="get" action="/listings#stays" role="search">
                <label class="hero-search-field">
                    <Icon name="pin" size={21} />
                    <span><strong>Where to?</strong><input type="search" name="q" value={filters.q} placeholder="City or stay" maxlength="100" /></span>
                </label>
                <label class="hero-search-field hero-category">
                    <Icon name="grid" size={19} />
                    <span>
                        <strong>Stay type</strong>
                        <select name="category" value={filters.category}>
                            <option value="">Any stay type</option>
                            {#each categories as category}<option value={category}>{category}</option>{/each}
                        </select>
                    </span>
                </label>
                {#each ['min', 'max', 'sort'] as field}
                    {#if filters[field]}<input type="hidden" name={field} value={filters[field]} />{/if}
                {/each}
                <button class="button primary"><Icon name="search" size={19} /> Search</button>
            </form>
        </div>
    </section>
    <nav class="category-nav" aria-label="Browse by stay type">
        <a href={catalogLink({ category: '', page: '' })} aria-current={filters.category ? undefined : 'true'}>
            <Icon name="grid" size={18} /> All stays
        </a>
        {#each categories as category}
            <a href={catalogLink({ category, page: '' })} aria-current={filters.category === category ? 'true' : undefined}>
                <Icon name={category} size={20} /> {category}
            </a>
        {/each}
    </nav>
{/if}

<section class="catalog" id="stays" aria-labelledby="catalog-heading">
    <div class="catalog-heading">
        <div>
            {#if explore}<h2 id="catalog-heading">{heading}</h2>{:else}<h1 id="catalog-heading">{heading}</h1>{/if}
            {#if mode === 'mine'}
                <p class="muted">Manage the stays you’ve published.</p>
            {:else if mode === 'saved'}
                <p class="muted">Keep your shortlist in one place.</p>
            {:else if catalog.count}
                <p class="muted">{catalog.count} {catalog.count === 1 ? 'stay' : 'stays'}{filters.q ? ` matching “${filters.q}”` : filters.category ? ` · ${filters.category}` : ''}</p>
            {/if}
        </div>
        {#if mode === 'mine'}<a class="button primary" href="/listings/new">Host a stay <Icon name="arrow" size={16} /></a>{/if}
        <details class="catalog-filters">
            <summary class="button"><Icon name="sliders" size={18} /> Filters & sorting</summary>
            <form class="filter-fields" method="get" action={`${base}#stays`} aria-label="Filter stays">
                {#if explore}
                    <input type="hidden" name="q" value={filters.q} />
                    <input type="hidden" name="category" value={filters.category} />
                {:else}
                    <label class="filter-search">Search stays <input type="search" name="q" value={filters.q} maxlength="100" /></label>
                    {#if filters.category}<input type="hidden" name="category" value={filters.category} />{/if}
                {/if}
                <label>Minimum price (₹)<input type="number" name="min" min="0" step="any" value={filters.min} /></label>
                <label>Maximum price (₹)<input type="number" name="max" min="0" step="any" value={filters.max} /></label>
                <label>
                    Sort by
                    <select name="sort" value={filters.sort || 'newest'}>
                        <option value="newest">Newest</option>
                        <option value="price-low">Price low to high</option>
                        <option value="price-high">Price high to low</option>
                    </select>
                </label>
                <button class="button primary">Apply filters</button>
            </form>
        </details>
    </div>

    {#if message || actionMessage}<p class="notice" role="alert">{actionMessage || message}</p>{/if}
    <p class="sr-only" role="status">{savedChange || ''}</p>
    <p class="sr-only" role="status">{announced}</p>

    {#if catalog.stays.length}
        <div class="catalog-meta">
            {#if filtering}<a href={`${base}#stays`}>Clear filters</a>{:else}<span></span>{/if}
            {#if mode !== 'mine'}<label class="check"><input type="checkbox" bind:checked={showTax} /> Include 18% tax</label>{/if}
        </div>
        <div class="stay-grid">
            {#each catalog.stays as stay, index (stay.id)}
                {@const isSaved = savedIds.includes(stay.id)}
                {@const compared = selected.includes(stay.id)}
                <article class="stay-card">
                    <div class="stay-media">
                        <a class="stay-photo-link" href={`/listings/${stay.id}`} tabindex="-1" aria-hidden="true">
                            <StayImage source={stay.image} loading={index < 3 ? 'eager' : 'lazy'} />
                        </a>
                        {#if mode !== 'mine'}
                            {#if signedIn}
                                <form class="card-save" method="post" action={catalogLink({}, true)} use:enhance={submitSave}>
                                    <input type="hidden" name="id" value={stay.id} />
                                    <input type="hidden" name="operation" value={isSaved ? 'unsave' : 'save'} />
                                    <button class="icon-button" aria-label={`${isSaved ? 'Remove saved' : 'Save'} ${stay.title}`}
                                        aria-pressed={isSaved} disabled={pending.includes(stay.id)}>
                                        <Icon name="heart" size={21} filled={isSaved} />
                                    </button>
                                </form>
                            {:else}
                                <a class="card-save icon-button" href={`/login?next=/listings/${stay.id}`} aria-label={`Sign in to save ${stay.title}`}><Icon name="heart" size={21} /></a>
                            {/if}
                        {/if}
                    </div>
                    <div class="stay-card-body">
                        {#if stay.category}<p class="stay-category">{stay.category}</p>{/if}
                        <h3><a href={`/listings/${stay.id}`}>{stay.title}</a></h3>
                        <p class="stay-location"><Icon name="pin" size={15} /> {stay.location}, {stay.country}</p>
                        <div class="stay-card-bottom">
                            <p><strong>{nightlyPrice(stay.price * (showTax ? 1.18 : 1))}</strong><span class="muted"> / night</span></p>
                            {#if mode === 'mine'}
                                <a class="card-compare" href={`/listings/${stay.id}/edit`}>Edit stay <Icon name="arrow" size={15} /></a>
                            {:else if ready}
                                <button type="button" class="card-compare" aria-label={`${compared ? 'Remove from comparison' : 'Compare'} ${stay.title}`}
                                    aria-pressed={compared} onclick={() => toggleComparison(stay.id)}>
                                    <Icon name="compare" size={17} /> {compared ? 'Added' : 'Compare'}
                                </button>
                            {/if}
                        </div>
                    </div>
                </article>
            {/each}
        </div>
        {#if catalog.pages > 1}
            <nav class="pagination" aria-label="Results pages">
                {#if catalog.page > 1}<a class="button" href={catalogLink({ page: String(catalog.page - 1) })}>Previous</a>{/if}
                <span>Page {catalog.page} of {catalog.pages}</span>
                {#if catalog.page < catalog.pages}<a class="button" href={catalogLink({ page: String(catalog.page + 1) })}>Next <Icon name="arrow" size={16} /></a>{/if}
            </nav>
        {/if}
    {:else if !message}
        <section class="empty">
            <h2>{filtering ? 'No stays match your filters' : mode === 'saved' ? 'Your shortlist is empty' : 'No stays published yet'}</h2>
            {#if filtering}
                <a class="button" href={`${base}#stays`}>Clear filters</a>
            {:else if mode === 'saved'}
                <p>Tap the heart on a stay to save it here.</p>
                <a class="button" href="/listings">Explore stays <Icon name="arrow" size={16} /></a>
            {:else}
                <a class="button" href="/listings/new">Host a stay <Icon name="arrow" size={16} /></a>
            {/if}
        </section>
    {/if}
</section>

{#if mode !== 'mine' && selected.length}
    <aside class="compare-dock" aria-label="Selected stays">
        {#if announced.startsWith('Clear or')}
            <p class="compare-limit" aria-hidden="true">Remove a stay or clear your selection to add another.</p>
        {/if}
        <div><Icon name="compare" size={22} /><span><strong>{selected.length} of {maxComparison}</strong><small> stays selected</small></span></div>
        <button type="button" class="text-button" onclick={() => setComparison([], 'Comparison cleared.')}>Clear</button>
        <a class="button primary" href={comparisonUrl(selected)}>Compare <Icon name="arrow" size={16} /></a>
    </aside>
{/if}
