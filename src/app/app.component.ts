import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent implements OnInit {
  title = 'G-Variables';

  ngOnInit(): void {
    invoke('greet2');
    console.log('teste');
  }
}
