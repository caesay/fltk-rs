#include <FL/Fl_Button.H>
#include "cfl_button.h"

Fl_Button *Fl_Button_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Button(x, y, width, height, title);
}

void Fl_Button_callback(Fl_Button *self, Fl_Callback* cb, void* data) {
    self->callback(cb, data);
}

void Fl_Button_set_label(Fl_Button *self, const char *title) {
    self->label(title);
}

void Fl_Button_redraw(Fl_Button *self) {
    self->redraw();
}