export async function load({ url }) {
    return {
        isLoggedIn: url.searchParams.get('logged_in') === 'true',
    }
}
