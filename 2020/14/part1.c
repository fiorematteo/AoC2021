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

struct value {
    unsigned long i;
    unsigned long v;
};

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    struct value values[500];
    memset(values, 0, sizeof(values));
    int values_len = 0;
    int mask[36];
    memset(mask, -1, sizeof(mask));
    while (getline(&line, &len, file) != EOF) {
        if (line[1] == 'a') {
            strsep(&line, " ");
            strsep(&line, " ");
            char *string_mask = strsep(&line, " ");
            string_mask[36] = '\0';
            printf("%lu\n", strlen(string_mask));
            for (int i = 0; i < strlen(string_mask); i++) {
                assert(35 - i >= 0);
                if (string_mask[35 - i] == '1') {
                    mask[i] = 1;
                } else if (string_mask[35 - i] == '0') {
                    mask[i] = 0;
                } else if (string_mask[35 - i] == 'X') {
                    mask[i] = -1;
                } else {
                    assert("bad char in string_mask" || false);
                }
            }
            printf("change of mask: ");
            for (int i = 0; i < 36; i++)
                if (mask[i] == -1)
                    printf("X");
                else
                    printf("%d", mask[i]);
            printf("\n");
        } else {
            strsep(&line, "[]");
            unsigned long index = atol(strsep(&line, "[]"));
            strsep(&line, "=");
            unsigned long value = atol(line);
            printf("value before %lu\n", value);
            for (int i = 0; i < 36; i++) {
                if (mask[i] == -1) {
                    continue;
                }
                if (mask[i] == 1) {
                    unsigned long a = 1UL << i;
                    printf("or  index: %d mask: %lu\n", i, a);
                    value |= a;
                } else if (mask[i] == 0) {
                    unsigned long a = ~(1UL << i);
                    printf("and index: %d mask: %lu\n", i, a);
                    value &= a;
                }
            }

            bool found = false;
            for (int i = 0; i < sizeof(values) / sizeof(struct value); i++) {
                if (values[i].i == index) {
                    values[i] = (struct value){index, value};
                    found = true;
                    break;
                }
            }
            if (!found) {
                values[values_len++] = (struct value){index, value};
                assert(values_len < sizeof(values) / sizeof(struct value));
            }
            printf("index %lu\n", index);
            printf("value after  %lu\n\n", value);
        }
        line = NULL;
    }
    printf("\n");
    unsigned long sum = 0;
    for (int i = 0; i < sizeof(values) / sizeof(struct value); i++) {
        if (values[i].v == 0)
            continue;
        sum += values[i].v;
        printf("%lu sum: %lu\n", values[i].v, sum);
    }
    printf("sum: %lu\n", sum);
}
