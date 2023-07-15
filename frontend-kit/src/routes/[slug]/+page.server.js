import { error } from '@sveltejs/kit';
import { dummyData } from '../data.js';

export function load({ params }) {
	const card = dummyData.find((card) => card.title === params.slug);
	if (!card) throw error(404);
	return {
		card
	};
}
