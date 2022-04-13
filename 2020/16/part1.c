#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

struct range {
    int start;
    int end;
    struct range *alternative;
};

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    struct range *ranges = malloc(sizeof(struct range) * 1000);
    int ranges_len = 0;
    while (getline(&line, &len, file) != EOF) {
        if (strcmp(line, "\n") == 0)
            break;
        strsep(&line, ":");
        int start1 = atoi(strsep(&line, "-"));
        int end1 = atoi(strsep(&line, " "));
        strsep(&line, "r");
        int start2 = atoi(strsep(&line, "-"));
        int end2 = atoi(strsep(&line, " "));
        ranges[ranges_len].start = start1;
        ranges[ranges_len].end = end1;
        ranges[ranges_len].alternative = malloc(sizeof(struct range));
        ranges[ranges_len].alternative->start = start2;
        ranges[ranges_len].alternative->end = end2;
        ranges[ranges_len].alternative->alternative = &ranges[ranges_len];
        ranges_len++;
    }
    getline(&line, &len, file);
    getline(&line, &len, file);
    getline(&line, &len, file);
    getline(&line, &len, file);

    int total = 0;
    while (getline(&line, &len, file) != EOF) {
        char *token;
        while ((token = strsep(&line, ",")) != NULL) {
            int value = atoi(token);
            int out_of_range = 0;
            for (int i = 0; i < ranges_len; i++) {
                if ((value >= ranges[i].start && value <= ranges[i].end) ||
                    (value >= ranges[i].alternative->start &&
                     value <= ranges[i].alternative->end)) {
                    continue;
                }
                out_of_range++;
            }
            if (out_of_range == ranges_len) {
                printf("%d\n", value);
                total += value;
            }
        }
    }
    printf("sum=%d\n", total);
}
