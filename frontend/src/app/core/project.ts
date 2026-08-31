import { Localized } from './i18n';

export interface Project {
  slug: string;
  name: string;
  category: Localized;
  status: Localized;
  year: number;
  description: Localized;
  technologies: string[];
  featured: boolean;
  repositoryUrl?: string;
  liveUrl?: string;
  imageUrl?: string;
  logoUrl?: string;
}
