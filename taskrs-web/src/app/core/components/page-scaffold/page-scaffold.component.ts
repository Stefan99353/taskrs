import {Component, OnInit} from '@angular/core';
import {FormControl} from '@angular/forms';
import {ThemeService} from '../../services/theme.service';
import {Theme} from '../../enums/theme';

@Component({
    selector: 'app-page-scaffold',
    templateUrl: './page-scaffold.component.html',
    styleUrls: ['./page-scaffold.component.scss'],
})
export class PageScaffoldComponent implements OnInit {
    isSidenavOpen = false;
    toggleDarkTheme = new FormControl(false);

    constructor(
        private _themeService: ThemeService,
    ) {
    }

    ngOnInit(): void {
        this.toggleDarkTheme.valueChanges.subscribe(toggleValue => {
            if (toggleValue === true) {
                this._themeService.switchTheme(Theme.Dark);
            } else {
                this._themeService.switchTheme(Theme.Light);
            }
        });

        const storedTheme = this._themeService.getCurrentTheme();
        switch (storedTheme) {
            case Theme.Dark:
                this.toggleDarkTheme.patchValue(true);
                break;
            case Theme.Light:
                this.toggleDarkTheme.patchValue(false);
                break;
        }
    }

    logout(): void {
        console.log('Logout');
    }
}
