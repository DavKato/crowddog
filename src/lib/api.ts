const basePath = 'https://app.crowdlog.jp';

class Api {
    constructor(private baseUrl: string) {}
    
    #call(path: string, options: RequestInit = {}) {
        return fetch(this.baseUrl + path, { ...options });
    }
    
    get(path: string, options: RequestInit = {}) {
        return this.#call(path, { ...options, method: 'GET' });
    }
    
    post(path: string, options: RequestInit = {}) {
        return this.#call(path, { ...options, method: 'POST' });
    }
    
    put(path: string, options: RequestInit = {}) {
        return this.#call(path, { ...options, method: 'PUT' });
    }
}

const api = new Api(basePath);

async function login(user: User) {
    const body = new FormData();
    body.append('email', user.email);
    body.append('passwd', user.passwd);
    body.append('auto', '1');
    body.append('rm', 'certify');

    const res = await api.post('/login.cgi', {
        body,
        mode: 'no-cors',
    });
    
    if (!res.ok) throw new Error('login failed')
}

export default {
    login,
}
