
/**
 * Dependencies
 * @ignore
 */

/**
 * Module Dependencies
 * @ignore
 */
import base64url from './base64url'

/**
 * Client
 * @ignore
 */
class Client {
    constructor(options = {}) {
        const defaults = {
            pathPrefix: '/webauthn',
            credential: {
                challengeEndpoint: '/credential/challenge',
                responseEndpoint: '/credential/response',
            },
            assertion: {
                challengeEndpoint: '/assertion/challenge',
                responseEndpoint: '/assertion/response',
            },
            logoutEndpoint: '/logout',
            userEndpoint: '/user'
        }

        Object.assign(this, defaults, options)
    }

    static publicKeyCredentialToJSON(pubKeyCred) {
        if (ArrayBuffer.isView(pubKeyCred)) {
            return Client.publicKeyCredentialToJSON(pubKeyCred.buffer)
        }

        if (pubKeyCred instanceof Array) {
            const arr = []

            for (let i of pubKeyCred) {
                arr.push(Client.publicKeyCredentialToJSON(i))
            }

            return arr
        }

        if (pubKeyCred instanceof ArrayBuffer) {
            return base64url.encode(pubKeyCred)
        }

        if (pubKeyCred instanceof Object) {
            const obj = {}

            for (let key in pubKeyCred) {
                obj[key] = Client.publicKeyCredentialToJSON(pubKeyCred[key])
            }

            return obj
        }

        return pubKeyCred
    }

    static generateRandomBuffer(len) {
        const buf = new Uint8Array(len || 32)
        window.crypto.getRandomValues(buf)
        return buf
    }

    static preformatMakeCredReq(makeCredReq) {
        makeCredReq.challenge = base64url.decode(makeCredReq.challenge)
        makeCredReq.user.id = base64url.decode(makeCredReq.user.id)
        return makeCredReq
    }

    static preformatGetAssertReq(getAssert) {
        getAssert.challenge = base64url.decode(getAssert.challenge)

        for (let allowCred of getAssert.allowCredentials) {
            allowCred.id = base64url.decode(allowCred.id)
        }

        return getAssert
    }

    async checkUser(formBody) {
        const response = await fetch(`${this.pathPrefix}${this.userEndpoint}`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(formBody)
        })

        if (response.status === 404) {
            console.log('checkUser - not found. Returning null')
            const failureMessage = (await response.json()).message
            const errorMessage = 'User not found'
            return null
        }

        if (response.status < 200 || response.status > 205) {
            throw new Error('Server responded with error.')
        }

        return await response.json()

    }

    async getMakeCredentialsChallenge(formBody) {
        const response = await fetch(`${this.pathPrefix}${this.credential.challengeEndpoint}`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(formBody)
        })

        if (response.status === 403) {
            const failureMessage = (await response.json()).message
            const errorMessage = 'Registration failed'
            throw new Error(failureMessage ? `${errorMessage}: ${failureMessage}.` : `${errorMessage}.`)
        }

        if (response.status < 200 || response.status > 205) {
            throw new Error('Server responded with error.')
        }

        return await response.json()
    }

    async sendWebAuthnResponse(ceremony, body) {
        console.log("Endpoint:", this[ceremony].responseEndpoint)
        const response = await fetch(`${this.pathPrefix}${this[ceremony].responseEndpoint}`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(body)
        })

        if (response.status !== 200) {
            throw new Error('Server responded with error.')
        }

        return await response.json()
    }

    async getGetAssertionChallenge(formBody) {
        const response = await fetch(`${this.pathPrefix}${this.assertion.challengeEndpoint}`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(formBody)
        })

        if (response.status !== 200) {
            throw new Error('Server responded with error.')
        }

        return await response.json()
    }

    async check(data = {}) {

    }

    async register(data = {}) {
        // Request challenge options from the RP
        const challenge = await this.getMakeCredentialsChallenge(data)
        // Base64 decode stuff,
        const publicKey = Client.preformatMakeCredReq(challenge)

        console.log('CREDENTIAL CHALLENGE', publicKey)

        const credential = await navigator.credentials.create({ publicKey })
        console.log('CREDENTIAL RESPONSE', credential)

        const credentialResponse = Client.publicKeyCredentialToJSON(credential)
        return await this.sendWebAuthnResponse('credential', credentialResponse)
    }

    async login(data = {}) {
        const challenge = await this.getGetAssertionChallenge(data)
        const publicKey = Client.preformatGetAssertReq(challenge)
        console.log('ASSERTION CHALLENGE', challenge)

        const credential = await navigator.credentials.get({ publicKey })
        console.log('ASSERTION RESPONSE', credentialResponse)
        const credentialResponse = Client.publicKeyCredentialToJSON(credential)
        return await this.sendWebAuthnResponse('assertion', credentialResponse)
    }

    async logout() {
        const response = await fetch(`${this.pathPrefix}${this.logoutEndpoint}`, {
            method: 'GET',
            credentials: 'include',
        })

        if (response.status !== 200) {
            throw new Error('Server responded with error.')
        }

        return await response.json()
    }
}

/**
 * Exports
 * @ignore
 */
export default Client