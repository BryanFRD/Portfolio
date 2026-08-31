import { Injectable, signal } from '@angular/core';

export type Locale = 'fr' | 'en';

export interface Localized<T = string> {
  fr: T;
  en: T;
}

@Injectable({
  providedIn: 'root',
})
export class LocaleService {
  readonly locale = signal<Locale>('fr');

  readonly t = <T>(value: Localized<T>): T => value[this.locale()];
}
