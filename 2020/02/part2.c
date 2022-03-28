#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_NAME "input"
//#define FILE_NAME "test"

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    int min, max;
    int validPasswords = 0;
    char *pass;
    char c;
    char *line;
    size_t len = 0;
    while (getline(&line, &len, file) != EOF) {
        sscanf(line, "%d-%d %c: %s\n", &min, &max, &c, pass);
        printf("%d-%d %c: %s\n", min, max, c, pass);
        int counter = 0;
        if (pass[min - 1] == c ^ pass[max - 1] == c)
            validPasswords++;
    }
    printf("%d\n", validPasswords);
}
