#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    int letters[26];
    memset(letters, 0, sizeof(int) * 26);
    int count = 0;
    int people = 0;
    while (getline(&line, &len, file) != EOF) {
        if (strcmp(line, "\n") == 0) {
            for (int i = 0; i < 26; i++) {
                if (letters[i] == people && people != 0) {
                    count++;
                }
                letters[i] = 0;
            }
            people = 0;
            continue;
        }
        people++;
        for (int i = 0; i < strlen(line); i++) {
            if (line[i] == '\n')
                continue;
            if (line[i] >= 'a' && line[i] <= 'z') {
                letters[line[i] - 'a']++;
            }
        }
    }
    for (int i = 0; i < 26; i++) {
        if (letters[i] == people && people != 0) {
            count++;
        }
    }
    printf("%d\n", count);
}
