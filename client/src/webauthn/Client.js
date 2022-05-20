
/**
 * Dependencies
 * @ignore
 */

/**
 * Module Dependencies
 * @ignore
 */
import base64url from './base64url'
//import base64url from 'base64url'
/**
 * Client
 * @ignore
 */
class Client {
    constructor(options = {}) {
        const defaults = {
            pathPrefix: '/webauthn',
            credentialEndpoint: '/register',
            assertionEndpoint: '/login',
            challengeEndpoint: '/response',
            logoutEndpoint: '/logout',
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
    /*
        static generateRandomBuffer(len) {
            const buf = new Uint8Array(len || 32)
            window.crypto.getRandomValues(buf)
            return buf
        }
    */
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

    /**
     * Send a request for a WebAuthn Credential Registration Challenge
     * The Verifier will return a publicKeyCredentialCreationOptions object
     * 
     * @param {*} formBody 
     * @returns 
     */
    async credentialRegistrationChallenge(formBody) {
        const response = await fetch(`${this.pathPrefix}${this.credentialEndpoint}`, {
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

    // Send a response to either a Credential (Registration) challenge or an
    // Assertion (authentication) challenge.  The server is responsible for 
    // tracking state to determine what challenge this is in response to.  For
    // our server, the challenge nonce is stored in the session, and matched against
    // the challenge send in the response. 
    async credentialResponse(body) {
        const response = await fetch(`${this.pathPrefix}${this.challengeEndpoint}`, {
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

    // Login Step 1: Send an assertion and receive  an Assertion Challenge
    async fetchAssertionChallenge(formBody) {

        const response = await fetch(`${this.pathPrefix}${this.assertionEndpoint}`, {
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

    async register(data = {}, journal) {
        // Credential Registration Challenge
        console.log('Registration Challenge >==>', data)
        const challenge = await this.credentialRegistrationChallenge(data)
        console.log('Registration Challenge <==<', challenge)

        const credentialCreationOptions = Client.preformatMakeCredReq(challenge)
        console.log('Verifier Public Key: ', credentialCreationOptions)

        const credential = await navigator.credentials.create({ credentialCreationOptions })
        console.log('Authenticator Credential: ', credential)

        const credentialResponse = Client.publicKeyCredentialToJSON(credential)
        console.log('Registration Response >==>', credentialResponse)

        // Credential Registration Response
        const response = await this.credentialResponse(credentialResponse)
        console.log('Registration Response <==<', response)
        return response

    }

    async login(data = {}) {
        console.log('Assertion Challenge >==>', data);
        const assertionChallenge = await this.fetchAssertionChallenge(data)
        console.log('Assertion Challenge <==<', assertionChallenge)

        const credentialRequestOptions = Client.preformatGetAssertReq(assertionChallenge)
        console.log('Verifier Public Key:', credentialRequestOptions)

        const assertion = await navigator.credentials.get({ credentialRequestOptions })
        console.log('Authenticator Assertion: ', assertion)

        const assertionResponse = Client.publicKeyCredentialToJSON(assertion)
        console.log('Assertion Response >==>', assertionResponse)

        const response = await this.credentialResponse(assertionResponse)
        console.log('Assertion Response <==<', response)
        return response
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
