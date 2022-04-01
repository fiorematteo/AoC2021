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

struct bus {
    int id;
    int offset;
};

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, file);
    getline(&line, &len, file);
    struct bus *buses = malloc(sizeof(struct bus) * len);
    int bus_len = 0;
    int off = 0;
    while (true) {
        char *str = strsep(&line, ",");
        if (!str)
            break;
        if (str[0] != 'x')
            buses[bus_len++] = (struct bus){atoi(str), off};
        off++;
    }

    long mcm = 1;
    long time = 0;
    for (int i = 1; i < bus_len; i++) {
        mcm *= buses[i - 1].id;
        while ((time + buses[i].offset) % buses[i].id != 0) {
            time += mcm;
        }
        printf("%ld\n", time);
    }
}
