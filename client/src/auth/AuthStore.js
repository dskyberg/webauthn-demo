import { makeAutoObservable, runInAction } from "mobx"
import { assertCredential } from '../webauthn'

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

    signin(name) {
        return assertCredential({ name })
            .then(response => {
                let json_response = JSON.parse(response)
                if (json_response.status == 'ok') {
                    console.log('login succeeded', json_response)
                    this.setUser(name)
                }
            }).catch(error => {
                console.log("Error:", error)
            })
    }

    signout() {
        this.setUser(null)
        // In case we want to interact with the auth server in the future
        return Promise.resolve(null)
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
