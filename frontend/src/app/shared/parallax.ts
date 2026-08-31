import {
  afterNextRender,
  Directive,
  ElementRef,
  inject,
  input,
  numberAttribute,
  OnDestroy,
} from '@angular/core';

@Directive({
  selector: '[appParallax]',
})
export class Parallax implements OnDestroy {
  private readonly elementRef = inject<ElementRef<HTMLElement>>(ElementRef);

  readonly strength = input(24, { alias: 'appParallax', transform: numberAttribute });

  private frame = 0;
  private bound = false;

  private readonly onScroll = (): void => {
    cancelAnimationFrame(this.frame);
    this.frame = requestAnimationFrame(() => this.update());
  };

  constructor() {
    afterNextRender(() => {
      if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
        return;
      }
      this.bound = true;
      this.update();
      window.addEventListener('scroll', this.onScroll, { passive: true });
    });
  }

  ngOnDestroy(): void {
    if (this.bound) {
      cancelAnimationFrame(this.frame);
      window.removeEventListener('scroll', this.onScroll);
    }
  }

  private update(): void {
    const viewport = window.innerHeight;
    if (viewport <= 0) {
      return;
    }
    const element = this.elementRef.nativeElement;
    const rect = element.getBoundingClientRect();
    const raw = (rect.top + rect.height / 2 - viewport / 2) / viewport;
    const progress = Math.max(-1, Math.min(1, raw));
    element.style.setProperty('--parallax-y', `${(-progress * this.strength()).toFixed(1)}px`);
  }
}
