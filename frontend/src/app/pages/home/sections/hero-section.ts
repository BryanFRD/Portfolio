import { afterNextRender, Component, inject, output, signal } from '@angular/core';
import { LocaleService } from '../../../core/i18n';
import { Parallax } from '../../../shared/parallax';

const NAME = 'Bryan Ferrando';

const TEXT = {
  role: {
    fr: 'Développeur Full-Stack & créateur de FerrLabs',
    en: 'Full-Stack Developer & creator of FerrLabs',
  },
  kicker: { fr: '[init] - bonjour, je suis', en: '[init] - hi, i am' },
  available: { fr: 'disponible', en: 'available' },
  seeProjects: { fr: 'voir les projets →', en: 'see the projects →' },
  contactMe: { fr: 'me contacter', en: 'contact me' },
};

type TypingPhase = 'idle' | 'name' | 'role' | 'done';

@Component({
  selector: 'app-hero-section',
  imports: [Parallax],
  templateUrl: './hero-section.html',
  styleUrl: './hero-section.scss',
})
export class HeroSection {
  readonly navigate = output<string>();

  protected readonly t = inject(LocaleService).t;
  protected readonly text = TEXT;
  protected readonly name = NAME;
  protected readonly role = this.t(TEXT.role);

  protected readonly line1 = signal(NAME);
  protected readonly line2 = signal(this.role);
  protected readonly phase = signal<TypingPhase>('done');
  protected readonly currentYear = new Date().getFullYear();

  constructor() {
    afterNextRender(() => {
      if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
        return;
      }
      this.line1.set('');
      this.line2.set('');
      this.phase.set('idle');
      setTimeout(() => this.typeName(), 300);
    });
  }

  private typeName(): void {
    this.phase.set('name');
    let i = 0;
    const interval = setInterval(() => {
      i++;
      this.line1.set(NAME.slice(0, i));
      if (i >= NAME.length) {
        clearInterval(interval);
        this.typeRole();
      }
    }, 55);
  }

  private typeRole(): void {
    this.phase.set('role');
    let i = 0;
    const interval = setInterval(() => {
      i++;
      this.line2.set(this.role.slice(0, i));
      if (i >= this.role.length) {
        clearInterval(interval);
        this.phase.set('done');
      }
    }, 35);
  }
}
