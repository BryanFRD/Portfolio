import { DOCUMENT } from '@angular/core';
import { TestBed } from '@angular/core/testing';
import { Meta, Title } from '@angular/platform-browser';
import { Seo } from './seo';
import { SITE_URL } from './site';

describe('Seo', () => {
  let service: Seo;
  let title: Title;
  let meta: Meta;
  let document: Document;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(Seo);
    title = TestBed.inject(Title);
    meta = TestBed.inject(Meta);
    document = TestBed.inject(DOCUMENT);
  });

  it('sets title, description and open graph tags for a page', () => {
    service.setPage({ title: 'Projets', description: 'Mes projets.', path: '/projects' });

    expect(title.getTitle()).toBe('Projets | Bryan Ferrando');
    expect(meta.getTag('name="description"')?.content).toBe('Mes projets.');
    expect(meta.getTag('property="og:url"')?.content).toBe(`${SITE_URL}/projects`);
    expect(meta.getTag('property="og:type"')?.content).toBe('website');
  });

  it('uses the site headline instead of a suffixed title on the home page', () => {
    service.setPage({ title: 'Accueil', path: '/' });

    expect(title.getTitle()).toBe('Bryan Ferrando | Développeur full-stack');
  });

  it('creates then updates a single canonical link', () => {
    service.setPage({ title: 'Projets', path: '/projects' });
    service.setPage({ title: 'Contact', path: '/contact' });

    const links = document.head.querySelectorAll('link[rel="canonical"]');
    expect(links.length).toBe(1);
    expect(links[0].getAttribute('href')).toBe(`${SITE_URL}/contact`);
  });

  it('replaces an existing JSON-LD script with the same id', () => {
    service.setJsonLd('person', { '@type': 'Person', name: 'A' });
    service.setJsonLd('person', { '@type': 'Person', name: 'B' });

    const scripts = document.head.querySelectorAll('script[type="application/ld+json"]');
    expect(scripts.length).toBe(1);
    expect(JSON.parse(scripts[0].textContent ?? '{}').name).toBe('B');
  });
});
