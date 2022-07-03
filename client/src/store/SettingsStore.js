import { action, observable, makeObservable } from "mobx"

export default class SettingsStore {

    isLoggedIn = false
    user = ''

    constructor() {
        makeObservable(this, {
            isLoggedIn: observable,
            user: observable,
            setUser: action,
            setIsLoggedIn: action,
            logout: action,
        })
    }
    setUser(value) {
        console.log('Settings - setUser:', value)
        this.user = value
    }

    setIsLoggedIn(value) {
        this.isLoggedIn = value
    }
    logout() {
        this.isLoggedIn = false
        this.user = ''
    }
}