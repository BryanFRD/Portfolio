import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { Terminal } from './shared/terminal/terminal';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, Terminal],
  templateUrl: './app.html',
  styleUrl: './app.scss',
})
export class App {}
