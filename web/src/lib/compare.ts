export const maxComparison = 3;
export const comparisonStorageKey = 'travaasa-comparison';

export function comparisonIds(values: unknown): string[] {
    if (!Array.isArray(values)) return [];
    return [...new Set(values.filter((value): value is string =>
        typeof value === 'string' && /^[a-f0-9]{24}$/i.test(value)
    ).map((value) => value.toLowerCase()))].slice(0, maxComparison);
}

export function comparisonUrl(ids: string[]) {
    const query = new URLSearchParams();
    for (const id of ids) query.append('stay', id);
    return `/listings/compare${query.size ? `?${query}` : ''}`;
}
