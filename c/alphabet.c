#include <stdio.h>

void main()
{
    char next;
    int abc[26] = {0};

    printf("Input a sentence: ");

    do
    {
        next = getchar();
        // puts into 0+ range for a
        int index = next - 'a';
        if (index > 26)
        {
            index = next - 'A';
        }

        if (0 <= index && index < 26)
        {
            abc[index] += 1;
        }

    } while (next != '\n');

    for (int i = 0; i < 26; i++)
    {
        if (abc[i] != 0)
        {
            printf("%c: %d\n", 'a' + i, abc[i]);
        }
    }
}