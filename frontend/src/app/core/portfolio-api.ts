import { HttpClient, httpResource, HttpResourceRef } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable, switchMap } from 'rxjs';
import { Project } from './project';

export interface ContactRequest {
  name: string;
  email: string;
  message: string;
}

@Injectable({
  providedIn: 'root',
})
export class PortfolioApi {
  private readonly http = inject(HttpClient);

  projects(): HttpResourceRef<Project[]> {
    return httpResource<Project[]>(() => '/api/projects', { defaultValue: [] });
  }

  sendContact(request: ContactRequest): Observable<void> {
    return this.http.get<{ token: string }>('/api/csrf').pipe(
      switchMap(({ token }) =>
        this.http.post<void>('/api/contact', request, {
          headers: { 'X-Csrf-Token': token },
        }),
      ),
    );
  }
}
