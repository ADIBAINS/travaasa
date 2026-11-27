<script lang="ts">
    import { nightlyPrice, reviewScore } from '#lib/stays.js';
    import Icon from '#lib/Icon.svelte';
    import ShareButton from '#lib/ShareButton.svelte';
    import type { PageProps } from './$types';
    let { data, form }: PageProps = $props();
    let stay = $derived(data.stay);
    let ownStay = $derived(data.user && data.user.id === stay.owner?.id);
    let rating = $derived(reviewScore(stay.reviews));
    let locationQuery = $derived(encodeURIComponent(`${stay.location}, ${stay.country}`));
</script>

<svelte:head><title>{stay.title} | Travaasa</title></svelte:head>

<a class="back-link" href="/listings">Explore stays</a>
{#if form?.message}<p class="notice" role="alert">{form.message}</p>{/if}
<article class="detail">
    <header class="detail-heading">
        <div>
            <h1>{stay.title}</h1>
            <p class="stay-location"><Icon name="pin" size={17} /> {stay.location}, {stay.country}{stay.category ? ` · ${stay.category}` : ''}</p>
        </div>
        <ShareButton />
    </header>
    {#if stay.image}
        <img class="hero-image" src={stay.image} alt={stay.title} fetchpriority="high" />
    {/if}
    <div class="detail-grid">
        <aside class="stay-summary" aria-label="Price and saved stay">
            <p class="summary-price"><strong>{nightlyPrice(stay.price)}</strong><span class="muted"> / night</span></p>
            {#if rating}<a class="summary-rating" href="#reviews"><span aria-hidden="true">★</span> {rating} · {stay.reviews.length} {stay.reviews.length === 1 ? 'review' : 'reviews'}</a>{/if}
            {#if data.user}
                <form method="post" action={data.saved ? '?/unsave' : '?/save'}>
                    <button class="button primary" aria-pressed={data.saved}><Icon name="heart" size={18} filled={data.saved} /> {data.saved ? 'Remove from saved' : 'Save stay'}</button>
                </form>
            {:else}
                <a class="button primary" href={`/login?next=/listings/${stay.id}`}><Icon name="heart" size={18} /> Sign in to save</a>
            {/if}
        </aside>
        <div class="detail-content">
            <section class="detail-section">
                <h2>{stay.owner ? `Hosted by ${stay.owner.username}` : 'About this stay'}</h2>
                <p class="description">{stay.description}</p>
                {#if ownStay}
                    <div class="owner-tools">
                        <a class="button" href={`/listings/${stay.id}/edit`}>Edit stay</a>
                        <details>
                            <summary>Delete stay</summary>
                            <form method="post" action="?/deleteStay" class="stack">
                                <label class="check"><input type="checkbox" name="confirm" value="yes" required /> Delete this stay and its reviews</label>
                                <button class="button danger">Confirm deletion</button>
                            </form>
                        </details>
                    </div>
                {/if}
            </section>
            <section class="detail-section">
                <h2>Where you’ll be</h2>
                {#if data.mapImage}
                    <img class="map-image" src={data.mapImage} alt={`Map near ${stay.location}`} loading="lazy" />
                {/if}
                <a class="map-link" href={`https://www.openstreetmap.org/search?query=${locationQuery}`}><Icon name="pin" size={17} /> Open map <Icon name="arrow" size={16} /></a>
            </section>
            <section class="detail-section" id="reviews">
                <h2>Reviews {stay.reviews.length ? `(${stay.reviews.length})` : ''}</h2>
                {#if stay.reviews.length === 0}<p class="muted">No reviews yet.</p>{/if}
                <div class="reviews">
                    {#each stay.reviews as review (review.id)}
                        <article class="review">
                            <p><strong>{review.author?.username ?? 'Former member'}</strong> · {review.rating}/5</p>
                            <p class="description">{review.comment}</p>
                            {#if data.user && data.user.id === review.author?.id}
                                <form method="post" action="?/deleteReview">
                                    <input type="hidden" name="reviewId" value={review.id} />
                                    <button class="text-button danger-text">Delete review</button>
                                </form>
                            {/if}
                        </article>
                    {/each}
                </div>
                {#if data.user}
                    <form class="stack review-form" method="post" action="?/review">
                        <h3>Leave a review</h3>
                        <label>
                            Rating
                            <select name="rating" required>
                                <option value="">Select a rating</option>
                                {#each [5, 4, 3, 2, 1] as stars}<option value={stars}>{stars} out of 5</option>{/each}
                            </select>
                        </label>
                        <label>Comment <textarea name="comment" rows="4" maxlength="2500" required></textarea></label>
                        <button class="button primary">Post review</button>
                    </form>
                {/if}
            </section>
        </div>
    </div>
</article>
