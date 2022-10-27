import { makeAutoObservable, runInAction } from "mobx"

export default class PolicyStore {

    isLoading = false;

    model = {
        rpId: '',
        rpName: '',
        keyType: '',
        alg: 0,
        authenticatorAttachment: '',
        authenticatorTransports: [],
        residentKey: '',
        userVerification: '',
        origin: '',
        attestation: '',
        timeout: 0,
        validateSignCount: false,
        defaultUserDisplayName: '',
        defaultUserName: '',
    }

    rpId = ''
    rpName = ''
    keyType = ''
    alg = 0
    authenticatorAttachment = ''
    authenticatorTransports = []
    residentKey = ''
    userVerification = ''
    origin = ''
    attestation = ''
    timeout = 0
    validateSignCount = false
    defaultUserDisplayName = ''
    defaultUserName = ''


    constructor() {
        makeAutoObservable(this, {}, { autoBind: true })
        this.loadModel()
    }

    loadModel() {
        this.isLoading = true;
        fetch('/api/policy', {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            }
        }).then(response => {
            return response.json()
        }).then(model => runInAction(() => {
            this.model = model
            this.updateFromModel()
            this.isLoading = false
            console.log(JSON.stringify(this.model))
        }))
    }

    patch() {
        let patchSet = {}
        for (const [key, value] of Object.entries(this.model)) {
            if (this[key] !== value) {
                patchSet[key] = this[key]
            }
        }

        this.isLoading = true;
        fetch('/policy', {
            method: 'PATCH',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(patchSet)
        }).then(response => {
            return response.json()
        }).then(model => runInAction(() => {
            this.model = model
            this.updateFromModel()
            this.isLoading = false
            console.log(JSON.stringify(this.model))
        }))

    }

    updateFromModel() {
        for (const [key, value] of Object.entries(this.model)) {
            if (this.dirty(key)) {
                this[key] = value
            }
        }
    }

    dirty(key) {
        return this[key] !== this.model[key]
    }

    reset(key) {
        console.log('Resetting', key)
        this[key] = this.model[key]
    }

    get isDirty() {
        return (
            this.dirty('rpid') ||
            this.dirty('rpName') ||
            this.dirty('keyType') ||
            this.dirty('alg') ||
            this.dirty('authenticatorAttachment') ||
            this.dirty('authenticatorTransports') ||
            this.dirty('residentKey') ||
            this.dirty('userVerification') ||
            this.dirty('origin') ||
            this.dirty('attestation') ||
            this.dirty('timeout') ||
            this.dirty('validateSignCount') ||
            this.dirty('defaultUserDisplayName') ||
            this.dirty('defaultUserName')
        )
    }

    setRpId(value) {
        this.rpId = value
    }

    setRpName(value) {
        this.rpName = value
    }

    setKeyType(value) {
        this.keyType = value
    }

    setAlg(value) {
        this.alg = value
    }

    setAuthenticatorAttachment(value) {
        this.authenticatorAttachment = value
    }

    setAuthenticatorTransports(values) {
        this.authenticatorTransports = values
    }

    setResidentKey(value) {
        this.residentKey = value
    }

    setUserVerification(value) {
        console.log('setting UserVerification:', value)

        this.userVerification = value
    }

    setOrigin(value) {
        this.origin = value
    }

    setAttestation(value) {
        this.attestation = value
    }

    setValidateSignCount(value) {
        console.log("validateSignCount:", value)
        this.validateSignCount = value
    }

    setTimeout(value) {
        this.timeout = value
    }
    setDefaultUserName(value) {
        this.defaultUserName = value
    }

    setDefaultUserDisplayName(value) {
        this.defaultUserDisplayName = value
    }
}


