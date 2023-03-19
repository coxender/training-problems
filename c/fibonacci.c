#include <stdio.h>

void main()
{
    unsigned int num1 = 0;
    unsigned int num2 = 1;

    for (int i = 0; i < 10; i++)
    {
        printf("%u \n", num1);
        num2 = num1 + num2;
        num1 = num2 - num1;
    }
}