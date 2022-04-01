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

void rotate(int *w_x, int *w_y, int angle) {
    angle /= 90;
    if (angle < 0)
        angle += 4;
    int tmp;
    switch (angle) {
    case 0:
        break;
    case 1:
        tmp = *w_x;
        *w_x = *w_y;
        *w_y = -tmp;
        break;
    case 2:
        *w_x = -*w_x;
        *w_y = -*w_y;
        break;
    case 3:
        tmp = *w_x;
        *w_x = -*w_y;
        *w_y = tmp;
        break;
    default:
        assert(false);
    }
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    int waypoint_x = 10;
    int waypoint_y = 1;
    int x = 0;
    int y = 0;
    int value = 0;
    char c;
    while (getline(&line, &len, file) != EOF) {
        sscanf(line, "%c%d", &c, &value);
        switch (c) {
        case 'F':
            x += value * waypoint_x;
            y += value * waypoint_y;
            break;
        case 'B':
            x -= value * waypoint_x;
            y -= value * waypoint_y;
            break;
        case 'L':
            rotate(&waypoint_x, &waypoint_y, -value);
            break;
        case 'R':
            rotate(&waypoint_x, &waypoint_y, value);
            break;
        case 'N':
            waypoint_y += value;
            break;
        case 'S':
            waypoint_y -= value;
            break;
        case 'W':
            waypoint_x -= value;
            break;
        case 'E':
            waypoint_x += value;
            break;
        }
    }
    printf("total_x: %d\n", x);
    printf("total_y: %d\n", y);
    printf("manhattan: %d\n", abs(x) + abs(y));
}
