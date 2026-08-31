import { provideZonelessChangeDetection } from '@angular/core';
import { ComponentFixture, TestBed } from '@angular/core/testing';
import { of, throwError } from 'rxjs';
import { vi } from 'vitest';
import { PortfolioApi } from '../../../core/portfolio-api';
import { ContactSection } from './contact-section';

describe('ContactSection', () => {
  let fixture: ComponentFixture<ContactSection>;
  let component: ContactSection;
  let sendContact: ReturnType<typeof vi.fn>;

  beforeEach(async () => {
    sendContact = vi.fn().mockReturnValue(of(undefined));
    await TestBed.configureTestingModule({
      imports: [ContactSection],
      providers: [
        provideZonelessChangeDetection(),
        { provide: PortfolioApi, useValue: { sendContact } },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(ContactSection);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  const fillValidForm = () => {
    component['form'].setValue({
      name: 'Jane Doe',
      email: 'jane@example.com',
      message: 'Bonjour, je souhaite discuter d’un projet.',
    });
  };

  it('does not call the API when the form is invalid', () => {
    component['form'].setValue({ name: '', email: 'not-an-email', message: 'court' });

    component['submit']();

    expect(sendContact).not.toHaveBeenCalled();
    expect(component['form'].touched).toBe(true);
  });

  it('sends the form payload and resets on success', () => {
    fillValidForm();

    component['submit']();

    expect(sendContact).toHaveBeenCalledWith({
      name: 'Jane Doe',
      email: 'jane@example.com',
      message: 'Bonjour, je souhaite discuter d’un projet.',
    });
    expect(component['state']()).toBe('sent');
    expect(component['form'].getRawValue().name).toBe('');
  });

  it('keeps the form values and reports an error when the API fails', () => {
    sendContact.mockReturnValue(throwError(() => new Error('boom')));
    fillValidForm();

    component['submit']();

    expect(component['state']()).toBe('error');
    expect(component['form'].getRawValue().email).toBe('jane@example.com');
  });
});
