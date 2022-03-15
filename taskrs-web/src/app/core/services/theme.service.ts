import {Injectable, Renderer2, RendererFactory2} from '@angular/core';
import {LocalStorageService} from './local-storage.service';
import {StorageKey} from '../enums/storage-key';
import {Theme} from '../enums/theme';

@Injectable({
    providedIn: 'root',
})
export class ThemeService {
    private _renderer: Renderer2;

    constructor(private _rendererFactory: RendererFactory2,
                private _localStorageService: LocalStorageService) {
        this._renderer = _rendererFactory.createRenderer(null, null);
    }

    public switchTheme(theme: Theme): void {
        for (const themeClass of Object.values(Theme)) {
            this._renderer.removeClass(document.body, themeClass);
        }

        this._renderer.addClass(document.body, theme);
        this._localStorageService.setValue(StorageKey.EnabledTheme, theme);
    }

    public getCurrentTheme(): Theme {
        const storedTheme = this._localStorageService.getValue(StorageKey.EnabledTheme);

        if (storedTheme) {
            return storedTheme;
        } else {
            const userPrefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
            let theme = userPrefersDark ? Theme.Dark : Theme.Light;

            this._localStorageService.setValue(StorageKey.EnabledTheme, theme);
            return theme;
        }
    }
}
