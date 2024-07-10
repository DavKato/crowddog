const initUser = () => {
    const saved = localStorage.getItem('user');
    return saved ?  JSON.parse(saved) as User : undefined;
}

class Store {
    #user: User | undefined = initUser();
    
    constructor() {};

    get user(): User | undefined {
        return this.#user;
    };

    set user(u: User) {
        localStorage.setItem('user', JSON.stringify(u));
        this.#user = u;
    }
};

export const store = new Store();
