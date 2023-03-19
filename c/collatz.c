#include <stdio.h>

void main()
{
    unsigned long int num = 63728127;

    while (num != 4) // ending for fancy math people
    {
        printf("%lu\n", num);
        if (num % 2 == 0)
        {
            num /= 2;
        }
        else
        {
            num = num * 3 + 1;
        }
    }
}