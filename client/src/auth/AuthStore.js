import { makeAutoObservable, runInAction } from "mobx"
import { FaWindows } from "react-icons/fa"

export default class AuthStore {
    user = null

    constructor() {
        makeAutoObservable(this)
        const user = sessionStorage.getItem("auth")
        if (user !== null) {
            this.user = JSON.parse(user)
        }
    }

    signin(user, callback = () => { }) {
        this.setUser(user)
        callback()
    }

    signout(callback = () => { }) {
        this.setUser(null)
        callback()
    }

    setUser(user) {
        console.log('Auth: setting user:', user)
        if (user !== null) {
            sessionStorage.setItem("auth", JSON.stringify(user))
        } else {
            sessionStorage.removeItem('auth')
        }
        this.user = user
    }

    get isLoggedIn() {
        return this.user != null
    }

}
