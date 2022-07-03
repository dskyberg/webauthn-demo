const ENDPOINTS = {
    credential: {
        challenge: '/webauthn/credential/challenge',
        response: '/webauthn/credential/response',
    },
    assertion: {
        challenge: '/webauthn/assertion/challenge',
        response: '/webauthn/assertion/response'
    },
    user: '/webauthn/user',
    credentials: '/webauthn/user/credentials',
}

const base64url_encode = (buffer) => {
    const base64String = window.btoa(new Uint8Array(buffer).reduce((data, byte) => {
        return data + String.fromCharCode(byte);
    }, ''));
    return base64String
}

const base64url_decode = (base64String) => {
    var base64 = (base64String)
        .replace(/-/g, '+')
        .replace(/_/g, '/');

    return new Uint8Array(window.atob(base64).split("").map(function (c) {
        return c.charCodeAt(0);
    }));
}

function publicKeyCredentialToJSON(pubKeyCred) {
    if (ArrayBuffer.isView(pubKeyCred)) {
        return publicKeyCredentialToJSON(pubKeyCred.buffer)
    }

    if (pubKeyCred instanceof Array) {
        const arr = []

        for (let i of pubKeyCred) {
            arr.push(publicKeyCredentialToJSON(i))
        }

        return arr
    }

    if (pubKeyCred instanceof ArrayBuffer) {
        return base64url_encode(pubKeyCred)
    }

    if (pubKeyCred instanceof Object) {
        const obj = {}

        for (let key in pubKeyCred) {
            obj[key] = publicKeyCredentialToJSON(pubKeyCred[key])
        }

        return obj
    }

    return pubKeyCred
}

async function sendWebAuthnChallenge(ceremony, formBody) {
    const response = await fetch(ENDPOINTS[ceremony].challenge, {
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

    let session = response.headers.get("X-SESSION")
    if (session != null) {
        sessionStorage.setItem("session", session)
    }

    return await response.json()
}


async function sendWebAuthnResponse(ceremony, body) {
    console.log("Endpoint:", ENDPOINTS[ceremony].response)
    let headers = { 'Content-Type': 'application/json' }

    let session = sessionStorage.getItem("session")
    if (session != null) {
        headers['X-SESSION'] = session
        sessionStorage.removeItem('session')
    }
    const response = await fetch(ENDPOINTS[ceremony].response, {
        method: 'POST',
        credentials: 'include',
        headers: headers,
        body: JSON.stringify(body)
    })

    if (response.status !== 200) {
        throw new Error('Server responded with error.')
    }

    return await response.json()
}


export async function createCredential(data = {}) {
    // Request challenge options from the RP
    const publicKey = await sendWebAuthnChallenge('credential', data)

    // Base64 decode stuff,
    publicKey.challenge = base64url_decode(publicKey.challenge)
    publicKey.user.id = base64url_decode(publicKey.user.id)

    console.log('CREDENTIAL CHALLENGE', publicKey)

    const credential = await navigator.credentials.create({ publicKey })
    console.log('CREDENTIAL RESPONSE', credential)

    const credentialResponse = publicKeyCredentialToJSON(credential)
    return await sendWebAuthnResponse('credential', credentialResponse)
}

export async function assertCredential(data = {}) {
    // Fetch the assertion options from the Verifier, and format it for
    // the CTAP Authenticator
    const publicKey = await sendWebAuthnChallenge('assertion', data)
    publicKey.challenge = base64url_decode(publicKey.challenge)

    for (let allowCred of publicKey.allowCredentials) {
        allowCred.id = base64url_decode(allowCred.id)
    }
    console.log('ASSERTION CHALLENGE', publicKey)

    // Call the CTAP Authenticator with the options
    const assertion = await navigator.credentials.get({ publicKey })
    const assertionResponse = publicKeyCredentialToJSON(assertion)

    console.log('ASSERTION RESPONSE', JSON.stringify(assertionResponse))
    return await sendWebAuthnResponse('assertion', assertionResponse)
}

export async function checkUser(formBody) {
    const response = await fetch(ENDPOINTS.user, {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(formBody)
    })

    if (response.status === 404) {
        console.log('checkUser - not found. Returning null')
        return null
    }

    if (response.status < 200 || response.status > 205) {
        throw new Error('Server responded with error.')
    }

    return await response.json()

}

export async function getUserCredentials(formBody) {
    const response = await fetch(ENDPOINTS.credentials, {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(formBody)
    })

    if (response.status === 404) {
        console.log('getUserCredentials - not found. Returning null')
        return null
    }

    if (response.status < 200 || response.status > 205) {
        throw new Error('Server responded with error.')
    }

    return await response.json()

}

