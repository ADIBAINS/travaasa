<script lang="ts">
    import { categories, nightlyPrice, type Catalog } from './stays.js';

    let { catalog, filters, base, saved, message }: {
        catalog: Catalog;
        filters: Record<string, string>;
        base: string;
        saved: boolean;
        message: string;
    } = $props();

    let showTax = $state(false);
    let filtering = $derived(Boolean(filters.q || filters.category || filters.min || filters.max));

    function pageLink(page: number) {
        const query = new URLSearchParams(Object.entries({ ...filters, page: page > 1 ? String(page) : '' })
            .filter(([, value]) => value));
        return query.size ? `${base}?${query}` : base;
    }
</script>

<svelte:head><title>{saved ? 'Saved stays' : 'Explore stays'} | Travaasa</title></svelte:head>

<h1>{saved ? 'Saved stays' : 'Explore stays'}</h1>
<form class="filters" method="get" action={base} role="search">
    <label class="destination">
        Destination or stay
        <input type="search" name="q" value={filters.q} placeholder="Search by place or name" maxlength="100" />
    </label>
    <label>
        Category
        <select name="category" value={filters.category}>
            <option value="">All categories</option>
            {#each categories as category}<option value={category}>{category}</option>{/each}
        </select>
    </label>
    <label>Min ₹<input type="number" name="min" min="0" step="any" value={filters.min} /></label>
    <label>Max ₹<input type="number" name="max" min="0" step="any" value={filters.max} /></label>
    <label>
        Sort by
        <select name="sort" value={filters.sort || 'newest'}>
            <option value="newest">Newest</option>
            <option value="price-low">Price low to high</option>
            <option value="price-high">Price high to low</option>
        </select>
    </label>
    <button class="button primary">Search</button>
</form>

{#if message}
    <p class="notice" role="alert">{message}</p>
{/if}

{#if catalog.count > 0}
    <div class="catalog-meta">
        <p>{catalog.count} {catalog.count === 1 ? 'stay' : 'stays'}{#if filtering} · <a href={base}>Clear filters</a>{/if}</p>
        <label class="check"><input type="checkbox" bind:checked={showTax} /> Show prices with 18% tax</label>
    </div>
    <div class="stay-grid">
        {#each catalog.stays as stay (stay.id)}
            <article class="stay-card">
                <a href={`/listings/${stay.id}`}>
                    {#if stay.image}
                        <img src={stay.image} alt="" loading="lazy" />
                    {:else}
                        <div class="no-image"><span>No photo</span></div>
                    {/if}
                    <h2>{stay.title}</h2>
                    <p class="muted">{stay.location}, {stay.country}</p>
                    <p><strong>{nightlyPrice(stay.price * (showTax ? 1.18 : 1))}</strong> / night</p>
                </a>
            </article>
        {/each}
    </div>
    {#if catalog.pages > 1}
        <nav class="pagination" aria-label="Results pages">
            {#if catalog.page > 1}<a class="button" href={pageLink(catalog.page - 1)}>Previous</a>{/if}
            <span>Page {catalog.page} of {catalog.pages}</span>
            {#if catalog.page < catalog.pages}<a class="button" href={pageLink(catalog.page + 1)}>Next</a>{/if}
        </nav>
    {/if}
{:else if !message}
    <section class="empty">
        <h2>{filtering ? 'No stays match your search' : saved ? 'No saved stays yet' : 'No stays published yet'}</h2>
        {#if filtering}
            <a href={base}>Clear filters</a>
        {:else if saved}
            <p>Save a stay from its listing page to find it here.</p>
            <a href="/listings">Explore stays</a>
        {/if}
    </section>
{/if}
