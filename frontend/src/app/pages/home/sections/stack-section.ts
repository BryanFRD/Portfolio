import { Component } from '@angular/core';

@Component({
  selector: 'app-stack-section',
  templateUrl: './stack-section.html',
  styleUrl: './stack-section.scss',
})
export class StackSection {
  protected readonly stack = [
    { category: 'Frontend', items: ['TypeScript', 'Angular', 'React', 'Sass', 'Tailwind CSS'] },
    { category: 'Backend', items: ['Rust', 'Axum', 'Node.js', 'Java', 'PHP', 'Python'] },
    { category: 'Jeux & 3D', items: ['Unity', 'C#'] },
    { category: 'Data & Infra', items: ['PostgreSQL', 'Docker', 'GitHub Actions', 'FerrFlow'] },
  ];

  protected readonly timeline = [
    { year: '2013', title: 'Premiers plugins Minecraft', tags: ['Java', 'Spigot'] },
    { year: '2016', title: 'Mods Minecraft', tags: ['Java', 'Forge'] },
    { year: '2022', title: 'Développement web', tags: ['JavaScript', 'PHP', 'React'] },
    { year: '2023', title: 'Entrée à Epitech', tags: ['React', 'Java', 'TypeScript'] },
    {
      year: '2024',
      title: 'Alternance chez Worldline',
      tags: ['Full-stack', 'MeTS', 'Secteur public'],
    },
    {
      year: '2026',
      title: 'Création de FerrLabs : FerrFlow, LFSX, IdleWarden',
      tags: ['Rust', 'Kubernetes', 'Angular'],
    },
    {
      year: '2026',
      title: 'Alternance chez Magellan, rachat de la branche MeTS',
      tags: ['Full-stack', 'MeTS', 'Secteur public'],
    },
  ];
}
