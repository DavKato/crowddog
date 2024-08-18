export async function load({ url }) {
	return {
		is_logged_in: url.searchParams.get('logged_in') === 'true',
	};
}
