import { Component, inject } from '@angular/core';
import { PortfolioApi } from '../../../core/portfolio-api';

@Component({
  selector: 'app-projects-section',
  templateUrl: './projects-section.html',
  styleUrl: './projects-section.scss',
})
export class ProjectsSection {
  protected readonly projects = inject(PortfolioApi).projects();

  protected pad(index: number): string {
    return String(index + 1).padStart(2, '0');
  }
}
