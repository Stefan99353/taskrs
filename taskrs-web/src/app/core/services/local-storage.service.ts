import {Injectable} from '@angular/core';

@Injectable({
    providedIn: 'root',
})
export class LocalStorageService {

    constructor() {
    }

    setValue(key: string, value: any): any {
        localStorage.setItem(key, JSON.stringify(value));
        return value;
    }

    getValue(key: string): any | null {
        const value = localStorage.getItem(key);

        if (value === null) {
            return null;
        }

        return JSON.parse(value);
    }

    removeValue(key: string): any | null {
        const value = this.getValue(key);
        localStorage.removeItem(key);
        return value;
    }
}
