import {Directive, ElementRef, Input, Renderer2} from '@angular/core';

@Directive({
  selector: '[bottom]'
})
export class BottomDirective {
  @Input('bottom') set changeBottomPosition(position: string | number) {
    this.renderer.setStyle(this.el.nativeElement, 'bottom', position)
  }

  constructor(private el: ElementRef, private renderer: Renderer2) {
  }
}
