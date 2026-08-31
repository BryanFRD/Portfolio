import { Component, inject } from '@angular/core';
import { LocaleService } from '../../../core/i18n';
import { PortfolioApi } from '../../../core/portfolio-api';
import { Parallax } from '../../../shared/parallax';

const TEXT = {
  title: { fr: 'Projets sélectionnés', en: 'Selected projects' },
  loading: { fr: 'chargement des projets…', en: 'loading projects…' },
  error: {
    fr: 'erreur : impossible de charger les projets.',
    en: 'error: could not load the projects.',
  },
  empty: { fr: 'aucun projet publié pour l’instant.', en: 'no project published yet.' },
};

@Component({
  selector: 'app-projects-section',
  imports: [Parallax],
  templateUrl: './projects-section.html',
  styleUrl: './projects-section.scss',
})
export class ProjectsSection {
  protected readonly t = inject(LocaleService).t;
  protected readonly text = TEXT;
  protected readonly projects = inject(PortfolioApi).projects();

  protected pad(index: number): string {
    return String(index + 1).padStart(2, '0');
  }
}
