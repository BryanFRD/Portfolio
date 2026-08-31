import { afterNextRender, Component, inject, signal } from '@angular/core';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { LocaleService } from '../../core/i18n';
import { Seo } from '../../core/seo';
import { GITHUB_URL, LINKEDIN_URL, SITE_NAME, SITE_URL } from '../../core/site';
import { ContactSection } from './sections/contact-section';
import { HeroSection } from './sections/hero-section';
import { ProjectsSection } from './sections/projects-section';
import { StackSection } from './sections/stack-section';

const NAV_LINKS = [
  { id: 'hero', label: { fr: 'index', en: 'index' } },
  { id: 'projets', label: { fr: 'projets', en: 'work' } },
  { id: 'stack', label: { fr: 'stack', en: 'stack' } },
  { id: 'contact', label: { fr: 'contact', en: 'contact' } },
];

const DESCRIPTION = {
  fr: 'Bryan Ferrando, développeur full-stack : alternant chez Magellan, étudiant à Epitech et créateur de FerrLabs. Projets, stack et contact.',
  en: 'Bryan Ferrando, full-stack developer: apprentice at Magellan, Epitech student and creator of FerrLabs. Projects, stack and contact.',
};

const FULL_TITLE = {
  fr: 'Bryan Ferrando | Développeur full-stack',
  en: 'Bryan Ferrando | Full-Stack Developer',
};

const JOB_TITLE = {
  fr: 'Développeur full-stack',
  en: 'Full-stack developer',
};

@Component({
  selector: 'app-home',
  imports: [RouterLink, HeroSection, ProjectsSection, StackSection, ContactSection],
  templateUrl: './home.html',
  styleUrl: './home.scss',
})
export class Home {
  private readonly seo = inject(Seo);
  private readonly localeService = inject(LocaleService);

  protected readonly t = this.localeService.t;
  protected readonly locale = this.localeService.locale;
  protected readonly navLinks = NAV_LINKS;
  protected readonly activeSection = signal('hero');
  protected readonly currentYear = new Date().getFullYear();

  constructor() {
    const isEnglish = inject(ActivatedRoute).snapshot.routeConfig?.path === 'en';
    this.localeService.locale.set(isEnglish ? 'en' : 'fr');
    const path = isEnglish ? '/en' : '/';

    this.seo.setPage({
      title: SITE_NAME,
      fullTitle: this.t(FULL_TITLE),
      description: this.t(DESCRIPTION),
      path,
      type: 'profile',
    });
    this.seo.setLocale(this.locale(), [
      { lang: 'fr', url: `${SITE_URL}/` },
      { lang: 'en', url: `${SITE_URL}/en` },
      { lang: 'x-default', url: `${SITE_URL}/` },
    ]);
    this.seo.setJsonLd('person', {
      '@context': 'https://schema.org',
      '@type': 'Person',
      name: SITE_NAME,
      url: SITE_URL,
      jobTitle: this.t(JOB_TITLE),
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
