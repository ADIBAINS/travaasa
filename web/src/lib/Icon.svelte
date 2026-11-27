<script lang="ts">
    let { name, size = 20, filled = false }: { name: string; size?: number; filled?: boolean } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill={filled ? 'currentColor' : 'none'} stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    {#if name === 'compass'}
        <circle cx="12" cy="12" r="9.25"/><path d="m15.8 8.2-2 5.6-5.6 2 2-5.6z"/>
    {:else if name === 'search'}
        <circle cx="10.75" cy="10.75" r="5.75"/><path d="m15 15 4.5 4.5"/>
    {:else if name === 'heart'}
        <path d="M20.8 8.9c0 4.1-8.8 9.5-8.8 9.5S3.2 13 3.2 8.9a4.5 4.5 0 0 1 8.8-1 4.5 4.5 0 0 1 8.8 1Z"/>
    {:else if name === 'arrow'}
        <path d="M4 12h15m-6-6 6 6-6 6"/>
    {:else if name === 'chevron'}
        <path d="m6 9 6 6 6-6"/>
    {:else if name === 'moon'}
        <path d="M20.4 15.5a9 9 0 0 1-11.9-12 8.9 8.9 0 1 0 11.9 12Z"/>
    {:else if name === 'sun'}
        <circle cx="12" cy="12" r="4"/><path d="M12 2v2m0 16v2M4.9 4.9l1.4 1.4m11.4 11.4 1.4 1.4M2 12h2m16 0h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/>
    {:else if name === 'grid'}
        <rect x="4" y="4" width="6" height="6" rx="1"/><rect x="14" y="4" width="6" height="6" rx="1"/><rect x="4" y="14" width="6" height="6" rx="1"/><rect x="14" y="14" width="6" height="6" rx="1"/>
    {:else if name === 'Beach'}
        <path d="M3 13a9 9 0 0 1 17-1 10 10 0 0 0-5 1 12 12 0 0 0-7 1 8 8 0 0 0-5-1ZM12 4l1-2m-1 11-2 7m-7 1h17"/>
    {:else if name === 'Mountains'}
        <path d="m2 19 8-13 3.1 5L16 7l6 12H2Zm5-8 3 3 2-2"/>
    {:else if name === 'Countryside'}
        <path d="m3 11 9-7 9 7m-16-1v10h14V10m-9 10v-5h4v5M18 8V5"/>
    {:else if name === 'City'}
        <path d="M3 21V9h6m0 12V3h7v18m0 0h5V12h-5M3 21h18M12 7h1m-1 4h1m-1 4h1m-1 4h1m-8-6h1m-1 4h1m12-1h1"/>
    {:else if name === 'compare'}
        <rect x="3" y="5" width="7" height="14" rx="1.5"/><rect x="14" y="5" width="7" height="14" rx="1.5"/><path d="M6 9h1m10 0h1"/>
    {:else if name === 'pin'}
        <path d="M19 10c0 5-7 10-7 10s-7-5-7-10a7 7 0 1 1 14 0Z"/><circle cx="12" cy="10" r="2.3"/>
    {:else if name === 'sliders'}
        <path d="M3 6h18M3 12h18M3 18h18M8 4v4m8 2v4m-6 2v4"/>
    {:else if name === 'share'}
        <path d="M12 15V3m-4 4 4-4 4 4M5 12v8h14v-8"/>
    {:else if name === 'close'}
        <path d="m6 6 12 12M18 6 6 18"/>
    {:else if name === 'photo'}
        <rect x="3" y="4" width="18" height="16" rx="2"/><circle cx="8" cy="9" r="1.5"/><path d="m4 16 4-4 3 3 5-5 5 6"/>
    {/if}
</svg>
