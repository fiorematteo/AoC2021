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
    int max_id = 0;
    while (getline(&line, &len, file) != EOF) {
        line[strlen(line) - 1] = '\0';
        int min_y = 0;
        int max_y = 127;
        int min_x = 0;
        int max_x = 7;
        int len_y, len_x;
        for (int i = 0; i < strlen(line); i++) {

            switch (line[i]) {
            case 'F':
                len_y = max_y - min_y;
                if (len_y % 2)
                    len_y++;
                max_y -= len_y / 2;
                break;
            case 'B':
                len_y = max_y - min_y;
                if (len_y % 2)
                    len_y++;
                min_y += len_y / 2;
                break;
            case 'L':
                len_x = max_x - min_x;
                if (len_x % 2)
                    len_x++;
                max_x -= len_x / 2;
                break;
            case 'R':
                len_x = max_x - min_x;
                if (len_x % 2)
                    len_x++;
                min_x += len_x / 2;
                break;
            default:
                assert(false);
            }
        }
        int id = min_y * 8 + min_x;
        if (id > max_id)
            max_id = id;
        printf("row: %d, col: %d, id %d\n", min_y, min_x, id);
    }
    printf("max id: %d\n", max_id);
}
