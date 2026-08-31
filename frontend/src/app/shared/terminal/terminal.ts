import { Component, ElementRef, signal, viewChild } from '@angular/core';
import { GITHUB_URL } from '../../core/site';

const WELCOME = 'Tapez "help" pour la liste des commandes.';

@Component({
  selector: 'app-terminal',
  templateUrl: './terminal.html',
  styleUrl: './terminal.scss',
  host: {
    '(document:keydown)': 'onDocumentKeydown($event)',
  },
})
export class Terminal {
  protected readonly open = signal(false);
  protected readonly lines = signal<string[]>([WELCOME]);

  private readonly promptInput = viewChild<ElementRef<HTMLInputElement>>('promptInput');

  protected onDocumentKeydown(event: KeyboardEvent): void {
    if ((event.key === '²' || event.key === '~') && !this.open()) {
      event.preventDefault();
      this.open.set(true);
      setTimeout(() => this.promptInput()?.nativeElement.focus());
    } else if (event.key === 'Escape' && this.open()) {
      this.open.set(false);
    }
  }

  protected close(): void {
    this.open.set(false);
  }

  protected run(input: HTMLInputElement): void {
    const command = input.value.trim();
    input.value = '';
    if (!command) {
      return;
    }
    this.print(`$ ${command}`);
    switch (command) {
      case 'help':
        this.print(
          'help      cette aide',
          'whoami    qui je suis',
          'projects  aller aux projets',
          'contact   aller au contact',
          'github    ouvrir mon GitHub',
          'clear     effacer',
          'exit      fermer',
        );
        break;
      case 'whoami':
        this.print('Bryan Ferrando : développeur full-stack (Worldline, Epitech, FerrLabs).');
        break;
      case 'projects':
        this.close();
        document.getElementById('projets')?.scrollIntoView({ behavior: 'smooth' });
        break;
      case 'contact':
        this.close();
        document.getElementById('contact')?.scrollIntoView({ behavior: 'smooth' });
        break;
      case 'github':
        window.open(GITHUB_URL, '_blank', 'noopener');
        break;
      case 'clear':
        this.lines.set([]);
        break;
      case 'exit':
        this.close();
        break;
      default:
        this.print(
          command.startsWith('sudo') ? 'Bien tenté.' : `commande introuvable : ${command}`,
        );
    }
  }

  private print(...newLines: string[]): void {
    this.lines.update((lines) => [...lines, ...newLines]);
  }
}
