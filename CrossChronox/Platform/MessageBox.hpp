#pragma once

#include "pch.hpp"

#if !defined(_WIN64) && !defined(_WIN32) // if not Windows
int MessageBoxA(sf::WindowHandle handle, const char* title, const char* text, unsigned flag);

enum : unsigned {
    // buttons
    MBOX_OK = 0b1,
    MBOX_YESNO = 0b10,
    MBOX_YESNOCANCEL = 0b100,

    // icon
    MBOX_ICONWARNING = 0b1000,
    MBOX_ICONINFORMATION = 0b10000,
    MBOX_ICONQUESTION = 0b100000,
    MBOX_ICONERROR = 0b1000000,
};

enum {
    BUTTON_CANCEL = -1,
    BUTTON_OK = 1,
    BUTTON_YES = 1,
    BUTTON_NO = 0,
};
#endif
