import { redirect } from '@sveltejs/kit';

export function load({ }) {
    if (!store.user) {
        redirect(307, '/login');
    }
}
