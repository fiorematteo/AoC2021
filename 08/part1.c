#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

int main() {
    FILE *f = fopen(FILE_NAME, "r");

    int count = 0;
    char *input = malloc(sizeof(char) * 200);
    size_t inputLen = 1000;
    while (getline(&input, &inputLen, f) != -1) {

        char *display[4];

        char *token = strtok(input, " ");
        for (int i = 0; i < 15; i++) {
            if (i > 10) {
                display[i - 11] = token;
            }
            token = strtok(NULL, " ");
        }
        display[3][strlen(display[3]) - 1] = '\0';
        for (int i = 0; i < 4; i++) {
            int len = strlen(display[i]);
            if (len == 2 || len == 3 || len == 4 || len == 7)
                count++;
        }
    }
    printf("final count: %d\n", count);
}
