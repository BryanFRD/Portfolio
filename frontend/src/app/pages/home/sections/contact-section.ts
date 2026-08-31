import { Component, inject, signal } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { PortfolioApi } from '../../../core/portfolio-api';
import { GITHUB_URL, LINKEDIN_URL } from '../../../core/site';

const EMAIL = 'bryanferrando59@gmail.com';

type SubmissionState = 'idle' | 'sending' | 'sent' | 'error';

@Component({
  selector: 'app-contact-section',
  imports: [ReactiveFormsModule],
  templateUrl: './contact-section.html',
  styleUrl: './contact-section.scss',
})
export class ContactSection {
  private readonly api = inject(PortfolioApi);
  private readonly formBuilder = inject(FormBuilder);

  protected readonly email = EMAIL;
  protected readonly copied = signal(false);
  protected readonly state = signal<SubmissionState>('idle');

  protected readonly form = this.formBuilder.nonNullable.group({
    name: ['', [Validators.required, Validators.maxLength(100)]],
    email: ['', [Validators.required, Validators.email]],
    message: ['', [Validators.required, Validators.minLength(10), Validators.maxLength(5000)]],
  });

  protected readonly profiles = [
    { label: 'github', value: 'github.com/BryanFRD', url: GITHUB_URL },
    { label: 'linkedin', value: 'linkedin.com/in/bryan-ferrando', url: LINKEDIN_URL },
  ];

  protected readonly statusRows = [
    { label: 'actuellement', value: 'alternant @ Worldline', highlight: false },
    { label: 'études', value: 'Epitech, MSc', highlight: false },
    { label: 'localisation', value: 'Hauts-de-France / remote', highlight: false },
    { label: 'ouvert aux', value: 'opportunités & projets', highlight: true },
  ];

  protected copy(): void {
    navigator.clipboard.writeText(EMAIL);
    this.copied.set(true);
    setTimeout(() => this.copied.set(false), 2000);
  }

  protected submit(): void {
    if (this.form.invalid) {
      this.form.markAllAsTouched();
      return;
    }
    this.state.set('sending');
    this.api.sendContact(this.form.getRawValue()).subscribe({
      next: () => {
        this.state.set('sent');
        this.form.reset();
      },
      error: () => this.state.set('error'),
    });
  }
}
