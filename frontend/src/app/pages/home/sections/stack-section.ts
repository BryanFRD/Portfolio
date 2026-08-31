import { Component, inject } from '@angular/core';
import { LocaleService } from '../../../core/i18n';

const TEXT = {
  label: { fr: '[03] - outils', en: '[03] - tools' },
  title: { fr: 'Stack technique', en: 'Tech stack' },
  timeline: { fr: '// parcours', en: '// journey' },
};

const STACK = [
  {
    category: { fr: 'Frontend', en: 'Frontend' },
    items: ['TypeScript', 'JavaScript', 'Angular', 'React', 'Astro', 'Sass', 'Tailwind CSS'],
  },
  {
    category: { fr: 'Backend', en: 'Backend' },
    items: ['Rust', 'Axum', 'SQLx', 'Node.js', 'Express', 'Java', 'Go', 'PHP', 'Python'],
  },
  {
    category: { fr: 'Jeux & 3D', en: 'Games & 3D' },
    items: ['Unity', 'C#'],
  },
  {
    category: { fr: 'Data', en: 'Data' },
    items: ['PostgreSQL', 'TimescaleDB', 'MongoDB', 'Redis'],
  },
  {
    category: { fr: 'Infra & CI/CD', en: 'Infra & CI/CD' },
    items: [
      'Docker',
      'Kubernetes',
      'FluxCD',
      'Helm',
      'Traefik',
      'Ansible',
      'GitHub Actions',
      'Grafana',
      'Prometheus',
    ],
  },
];

const TIMELINE = [
  {
    year: '2013',
    title: { fr: 'Premiers plugins Minecraft', en: 'First Minecraft plugins' },
    tags: { fr: ['Java', 'Spigot'], en: ['Java', 'Spigot'] },
  },
  {
    year: '2016',
    title: { fr: 'Mods Minecraft', en: 'Minecraft mods' },
    tags: { fr: ['Java', 'Forge'], en: ['Java', 'Forge'] },
  },
  {
    year: '2022',
    title: { fr: 'Développement web', en: 'Web development' },
    tags: { fr: ['JavaScript', 'PHP', 'React'], en: ['JavaScript', 'PHP', 'React'] },
  },
  {
    year: '2023',
    title: { fr: 'Entrée à Epitech', en: 'Joined Epitech' },
    tags: { fr: ['React', 'Java', 'TypeScript'], en: ['React', 'Java', 'TypeScript'] },
  },
  {
    year: '2024',
    title: { fr: 'Alternance chez Worldline', en: 'Apprenticeship at Worldline' },
    tags: {
      fr: ['Full-stack', 'MeTS', 'Secteur public'],
      en: ['Full-stack', 'MeTS', 'Public sector'],
    },
  },
  {
    year: '2026',
    title: {
      fr: 'Création de FerrLabs : FerrFlow, LFSX, IdleWarden',
      en: 'Founded FerrLabs: FerrFlow, LFSX, IdleWarden',
    },
    tags: { fr: ['Rust', 'Kubernetes', 'Angular'], en: ['Rust', 'Kubernetes', 'Angular'] },
  },
  {
    year: '2026',
    title: {
      fr: 'Alternance chez Magellan, rachat de la branche MeTS',
      en: 'Apprenticeship at Magellan, MeTS branch takeover',
    },
    tags: {
      fr: ['Full-stack', 'MeTS', 'Secteur public'],
      en: ['Full-stack', 'MeTS', 'Public sector'],
    },
  },
];

@Component({
  selector: 'app-stack-section',
  templateUrl: './stack-section.html',
  styleUrl: './stack-section.scss',
})
export class StackSection {
  protected readonly t = inject(LocaleService).t;
  protected readonly text = TEXT;
  protected readonly stack = STACK;
  protected readonly timeline = TIMELINE;
}
