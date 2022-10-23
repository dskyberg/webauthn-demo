import { makeAutoObservable, runInAction } from "mobx"

export default class UsersStore {
    isLoading = false;
    users = [];

    constructor() {
        makeAutoObservable(this, {}, { autoBind: true })
    }

    loadUsers() {
        this.isLoading = true;
        fetch('/users', {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            }
        }).catch(err => {
            this.isLoading = false
            console.log('loadUsers failed:', err)
        }).then(response => {
            if (response.status !== 200) {
                console.log('LoadUsers: Error status returned')
                return
            }
            return response.json()
        }).then(users => runInAction(() => {
            this.users = users
            this.isLoading = false
            console.log('UserStore - loaded users:', users)
        }))
    }

}
