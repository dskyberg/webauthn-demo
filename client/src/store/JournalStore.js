import { action, observable, makeAutoObservable, toJS } from "mobx"

export default class JournalStore {

    journal = []

    constructor() {
        makeAutoObservable(this)
    }

    log(group, message) {
        this.journal.push({ group, message })
    }

    journal2JS() {
        return toJS(this.journal)
    }
}