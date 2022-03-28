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
    bool letters[26];
    memset(letters, false, sizeof(char) * 26);
    int count = 0;
    while (getline(&line, &len, file) != EOF) {
        if (strcmp(line, "\n") == 0) {
            for (int i = 0; i < 26; i++) {
                count += letters[i];
                letters[i] = false;
            }
            continue;
        }
        for (int i = 0; i < strlen(line); i++) {
            if (line[i] == '\n')
                continue;
            if (line[i] >= 'a' && line[i] <= 'z') {
                letters[line[i] - 'a'] = true;
            }
        }
    }
    for (int i = 0; i < 26; i++)
        count += letters[i];
    printf("%d\n", count);
}
