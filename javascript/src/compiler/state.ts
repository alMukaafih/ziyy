import { RESET } from "../value"

export class State {
    tags: string[]
    saves: string[]

    constructor() {
        this.tags = [""]
        this.saves = [RESET]
    }

    push(tag: string, string: string) {
        const l = this.saves.length - 1
        const s = this.saves[l]
        this.saves.push(s + string)
        this.tags.push(tag)
    }

    pop(): string[] {
        const a = this.tags.pop()
        const b = this.saves.pop()
        return [a, b]
    }

    currentTag(): string {
        const l = this.tags.length - 1
        return this.tags[l]
    }

    currentSave(): string {
        const l = this.saves.length - 1
        return this.saves[l]
    }
}