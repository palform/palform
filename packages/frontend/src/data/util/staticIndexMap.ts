/**
 * Keep track of the original positions inside a list.
 * Helps to assign Svelte keys when using #each in dynamically changing lists
 * of items that don't inherently have unique IDs.
 */
export default class StaticIndexMap {
    private keys: number[];
    private nextKey: number;

    constructor(initialValues: any[]) {
        this.keys = Object.keys(initialValues).map((e) => parseInt(e));
        this.nextKey = initialValues.length;
    }

    insert() {
        this.keys.push(this.nextKey);
        this.nextKey++;
    }

    move(oldIndex: number, newIndex: number) {
        const newKeys = [...this.keys];
        const [removedKey] = newKeys.splice(oldIndex, 1);
        newKeys.splice(newIndex, 0, removedKey);
        this.keys = newKeys;
    }

    delete(index: number) {
        this.keys.splice(index, 1);
    }

    getKey(index: number) {
        return this.keys[index];
    }
}
