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

enum dir { NORTH, EAST, SOUTH, WEST };

void move_forward(enum dir dir, int value, int *total_x, int *total_y) {
    switch (dir) {
    case NORTH:
        *total_y += value;
        break;
    case SOUTH:
        *total_y -= value;
        break;
    case WEST:
        *total_x -= value;
        break;
    case EAST:
        *total_x += value;
        break;
    }
}

void rotate(enum dir *dir, int angle) {
    angle /= 90;
    if (angle > 0) {
        for (int i = 0; i < angle; i++) {
            *dir = (*dir + 1) % 4;
        }
    } else {
        angle = -angle;
        for (int i = 0; i < angle; i++) {
            *dir = (*dir - 1) % 4;
        }
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
    enum dir current_direction = EAST;
    int total_x = 0;
    int total_y = 0;
    int value = 0;
    char c;
    while (getline(&line, &len, file) != EOF) {
        sscanf(line, "%c%d", &c, &value);
        switch (c) {
        case 'F':
            move_forward(current_direction, value, &total_x, &total_y);
            break;
        case 'B':
            move_forward(current_direction, -value, &total_x, &total_y);
            break;
        case 'L':
            rotate(&current_direction, -value);
            break;
        case 'R':
            rotate(&current_direction, value);
            break;
        case 'N':
            total_y += value;
            break;
        case 'S':
            total_y -= value;
            break;
        case 'W':
            total_x -= value;
            break;
        case 'E':
            total_x += value;
            break;
        }
    }
    printf("total_x: %d\n", total_x);
    printf("total_y: %d\n", total_y);
    printf("manhattan: %d\n", abs(total_x) + abs(total_y));
}
