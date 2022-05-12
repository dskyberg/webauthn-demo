import { action, observable, makeObservable } from "mobx"

export default class SettingsStore {

    isLoggedIn = false

    constructor() {
        makeObservable(this, {
            isLoggedIn: observable,
            setIsLoggedIn: action,
        })
    }

    setIsLoggedIn(value) {
        this.isLoggedIn = value
    }
}