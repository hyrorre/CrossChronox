#pragma once

#include "pch.hpp"

#if !defined(_WIN64) && !defined(_WIN32) // if not Windows
int MessageBoxA(sf::WindowHandle handle, const char* text, const char* title, unsigned flag);

enum : long {
    // buttons
    MB_OK = 0x00000000L,
    MB_OKCANCEL = 0x00000001L,
    MB_ABORTRETRYIGNORE = 0x00000002L,
    MB_YESNOCANCEL = 0x00000003L,
    MB_YESNO = 0x00000004L,
    MB_RETRYCANCEL = 0x00000005L,
    MB_CANCELTRYCONTINUE = 0x00000006L,

    // icon
    MB_ICONSTOP = 0x00000010L,
    MB_ICONERROR = 0x00000010L,
    MB_ICONHAND = 0x00000010L,
    MB_ICONQUESTION = 0x00000020L,
    MB_ICONEXCLAMATION = 0x00000030L,
    MB_ICONWARNING = 0x00000030L,
    MB_ICONINFORMATION = 0x00000040L,
    MB_ICONASTERISK = 0x00000040L,
};

enum {
    BUTTON_CANCEL = -1,
    BUTTON_OK = 1,
    BUTTON_YES = 1,
    BUTTON_NO = 0,
};
#endif
