export const categories = ['Beach', 'Mountains', 'Countryside', 'City'];
const currency = new Intl.NumberFormat('en-IN', { style: 'currency', currency: 'INR', minimumFractionDigits: 0, maximumFractionDigits: 2 });

export const nightlyPrice = (price: number) => currency.format(price);

export type User = { id: string; username: string };
export type Stay = {
    id: string;
    title: string;
    location: string;
    country: string;
    category: string | null;
    price: number;
    image: string | null;
};
export type Detail = Stay & {
    description: string;
    owner: User | null;
    coordinates: [number, number] | null;
    reviews: { id: string; author: User | null; rating: number; comment: string }[];
};
export type Catalog = { stays: Stay[]; count: number; page: number; pages: number };

export function reviewScore(reviews: Detail['reviews']) {
    if (!reviews.length) return null;
    return (reviews.reduce((total, review) => total + review.rating, 0) / reviews.length).toFixed(1);
}
