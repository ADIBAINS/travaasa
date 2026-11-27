<script lang="ts">
    import Icon from './Icon.svelte';

    let feedback = $state('');

    async function copyLink() {
        try {
            await navigator.clipboard.writeText(location.href);
            feedback = 'Link copied.';
        } catch {
            feedback = 'Copy the link from your address bar.';
        }
    }
</script>

<div class="share-control">
    <button type="button" class="button" onclick={copyLink}><Icon name="share" size={17} /> Copy link</button>
    <span role="status">{feedback}</span>
</div>
