export interface Project {
  slug: string;
  name: string;
  category: string;
  status: string;
  year: number;
  description: string;
  technologies: string[];
  featured: boolean;
  repositoryUrl?: string;
  liveUrl?: string;
  imageUrl?: string;
  logoUrl?: string;
}
