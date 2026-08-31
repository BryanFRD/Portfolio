import { Component, inject, signal } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { LocaleService } from '../../../core/i18n';
import { PortfolioApi } from '../../../core/portfolio-api';
import { GITHUB_URL, LINKEDIN_URL } from '../../../core/site';

const EMAIL = 'bryanferrando59@gmail.com';

const TEXT = {
  label: { fr: '[04] - contact', en: '[04] - contact' },
  title: { fr: 'Travaillons ensemble', en: "Let's work together" },
  intro: {
    fr: "Ouvert aux opportunités, aux collaborations techniques et aux projets intéressants. N'hésitez pas à m'écrire, réponse rapide garantie.",
    en: 'Open to opportunities, technical collaborations and interesting projects. Feel free to write to me, quick reply guaranteed.',
  },
  sent: {
    fr: 'message envoyé, merci ! je vous réponds rapidement.',
    en: 'message sent, thank you! i will get back to you shortly.',
  },
  nameLabel: { fr: 'nom', en: 'name' },
  nameError: { fr: 'votre nom est requis.', en: 'your name is required.' },
  emailLabel: { fr: 'e-mail', en: 'e-mail' },
  emailError: {
    fr: 'une adresse e-mail valide est requise.',
    en: 'a valid e-mail address is required.',
  },
  messageLabel: { fr: 'message', en: 'message' },
  messageError: {
    fr: "un message d'au moins 10 caractères est requis.",
    en: 'a message of at least 10 characters is required.',
  },
  failure: {
    fr: "l'envoi a échoué. réessayez plus tard ou écrivez-moi directement par e-mail.",
    en: 'sending failed. try again later or write to me directly by e-mail.',
  },
  sending: { fr: 'envoi...', en: 'sending...' },
  send: { fr: 'envoyer le message →', en: 'send the message →' },
  copy: { fr: 'copier', en: 'copy' },
  copied: { fr: 'copié !', en: 'copied!' },
  visit: { fr: 'visiter', en: 'visit' },
  statusTitle: { fr: '// statut actuel', en: '// current status' },
};

const STATUS_ROWS = [
  {
    label: { fr: 'actuellement', en: 'currently' },
    value: { fr: 'alternant @ Magellan', en: 'apprentice @ Magellan' },
    highlight: false,
  },
  {
    label: { fr: 'études', en: 'studies' },
    value: { fr: 'Epitech, MSc', en: 'Epitech, MSc' },
    highlight: false,
  },
  {
    label: { fr: 'localisation', en: 'location' },
    value: { fr: 'Hauts-de-France / remote', en: 'Hauts-de-France, France / remote' },
    highlight: false,
  },
  {
    label: { fr: 'ouvert aux', en: 'open to' },
    value: { fr: 'opportunités & projets', en: 'opportunities & projects' },
    highlight: true,
  },
];

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

  protected readonly t = inject(LocaleService).t;
  protected readonly text = TEXT;
  protected readonly statusRows = STATUS_ROWS;
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
