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
    getline(&line, &len, file);
    int start = atoi(line);
    getline(&line, &len, file);
    int min = start + 100;
    int best_id;
    while (true) {
        char *str = strsep(&line, ",");
        if (!str)
            break;
        if (str[0] == 'x')
            continue;
        int bus_id = atoi(str);
        int departure_time = 0;
        while (departure_time < start) {
            departure_time += bus_id;
        }
        if (departure_time - start < min){
            min = departure_time - start;
            best_id = bus_id;
        }
    }
    printf("wait_time %d\n", min);
    printf("bus_id %d\n", best_id);
    printf("result %d\n", min * best_id);
}
