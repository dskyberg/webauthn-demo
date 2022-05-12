import { action, observable, makeObservable } from "mobx"

export default class JournalStore {

    journal = []

    constructor() {
        makeObservable(this, {
            journal: observable,
            log: action,
        })
    }

    log(group, message) {
        this.journal.push({ group, message })
    }

}