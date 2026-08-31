import { afterNextRender, Component, inject, signal } from '@angular/core';
import { Seo } from '../../core/seo';
import { GITHUB_URL, LINKEDIN_URL, SITE_NAME, SITE_URL } from '../../core/site';
import { ContactSection } from './sections/contact-section';
import { HeroSection } from './sections/hero-section';
import { ProjectsSection } from './sections/projects-section';
import { StackSection } from './sections/stack-section';

const NAV_LINKS = [
  { id: 'hero', label: 'index' },
  { id: 'projets', label: 'projets' },
  { id: 'stack', label: 'stack' },
  { id: 'contact', label: 'contact' },
];

@Component({
  selector: 'app-home',
  imports: [HeroSection, ProjectsSection, StackSection, ContactSection],
  templateUrl: './home.html',
  styleUrl: './home.scss',
})
export class Home {
  private readonly seo = inject(Seo);

  protected readonly navLinks = NAV_LINKS;
  protected readonly activeSection = signal('hero');
  protected readonly currentYear = new Date().getFullYear();

  constructor() {
    this.seo.setPage({
      title: 'Accueil',
      description:
        'Bryan Ferrando, développeur full-stack : alternant chez Magellan, étudiant à Epitech et créateur de FerrLabs. Projets, stack et contact.',
      path: '/',
      type: 'profile',
    });
    this.seo.setJsonLd('person', {
      '@context': 'https://schema.org',
      '@type': 'Person',
      name: SITE_NAME,
      url: SITE_URL,
      jobTitle: 'Développeur full-stack',
      worksFor: { '@type': 'Organization', name: 'Magellan Partners' },
      alumniOf: { '@type': 'EducationalOrganization', name: 'Epitech' },
      sameAs: [GITHUB_URL, LINKEDIN_URL],
    });

    afterNextRender(() => {
      const observer = new IntersectionObserver(
        (entries) => {
          for (const entry of entries) {
            if (entry.isIntersecting) {
              this.activeSection.set(entry.target.id);
            }
          }
        },
        { threshold: 0.3 },
      );
      for (const link of NAV_LINKS) {
        const element = document.getElementById(link.id);
        if (element) {
          observer.observe(element);
        }
      }
    });
  }

  protected scrollTo(id: string): void {
    document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' });
  }
}
