import { afterNextRender, Component, output, signal } from '@angular/core';

const NAME = 'Bryan Ferrando';
const ROLE = 'Développeur Full-Stack & créateur de FerrLabs';

type TypingPhase = 'idle' | 'name' | 'role' | 'done';

@Component({
  selector: 'app-hero-section',
  templateUrl: './hero-section.html',
  styleUrl: './hero-section.scss',
})
export class HeroSection {
  readonly navigate = output<string>();

  protected readonly line1 = signal(NAME);
  protected readonly line2 = signal(ROLE);
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
      this.line2.set(ROLE.slice(0, i));
      if (i >= ROLE.length) {
        clearInterval(interval);
        this.phase.set('done');
      }
    }, 35);
  }
}
