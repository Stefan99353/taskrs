import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {PageScaffoldComponent} from './core/components/page-scaffold/page-scaffold.component';
import {SharedModule} from './shared/shared.module';
import {MaterialModule} from './shared/material.module';

@NgModule({
    declarations: [
        AppComponent,
        PageScaffoldComponent,
    ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        BrowserAnimationsModule,
        MaterialModule,
        SharedModule,
    ],
    providers: [],
    bootstrap: [AppComponent],
})
export class AppModule {
}

