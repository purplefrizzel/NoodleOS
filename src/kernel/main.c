#include "main.h"

void kernel()
{
    printClear();
    printSetColor(PRINT_COLOR_MAGENTA, PRINT_COLOR_BLACK);
    printStr("NoodleOS - 0.1.0\n");

    printLogo();
}