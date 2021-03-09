#include "print.h"

void kernel()
{
    printClear();
    printSetColor(PRINT_COLOR_YELLOW, PRINT_COLOR_BLACK);
    printStr("Hello world!");
}