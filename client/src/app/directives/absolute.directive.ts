import {Directive, ElementRef} from '@angular/core';

@Directive({
  selector: '[absolute]'
})

export class AbsoluteDirective {
  constructor(el: ElementRef) {
    el.nativeElement.style.position = 'absolute'
  }
}
