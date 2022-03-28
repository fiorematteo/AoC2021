#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define MAX_Y 137
#define MAX_X 139

char directions[2] = {'>', 'v'};

void printBoard(char **b) {
    for (int i = 0; i < MAX_Y; i++) {
        for (int j = 0; j < MAX_X; j++)
            printf("%c", b[i][j]);
        printf("\n");
    }
}

bool move(char **b, char **c, char turn) {
    bool moved = false;
    for (int y = 0; y < MAX_Y; y++) {
        for (int x = 0; x < MAX_X; x++) {
            if (turn != b[y][x])
                continue;
            int newX = x, newY = y;
            switch (b[y][x]) {
            case '>':
                newX = x + 1;
                if (newX >= MAX_X)
                    newX = 0;
                break;
            case 'v':
                newY = y + 1;
                if (newY >= MAX_Y)
                    newY = 0;
                break;
            }
            if (b[newY][newX] == '.') {
                c[newY][newX] = turn;
                c[y][x] = '.';
                moved = true;
            }
        }
    }
    return moved;
}

int main() {
    FILE *fs = fopen(FILE_NAME, "r");
    assert(fs);

    char **board = malloc(MAX_Y * sizeof(char *));
    for (int i = 0; i < MAX_Y; i++)
        board[i] = malloc(MAX_X * sizeof(char));
    char **copy = malloc(MAX_Y * sizeof(char *));
    for (int i = 0; i < MAX_Y; i++)
        copy[i] = malloc(MAX_X * sizeof(char));

    char *line = NULL;
    size_t len = 0;
    int index = 0;
    while (getline(&line, &len, fs) != -1) {
        memcpy(board[index], line, MAX_X);
        index++;
        line = NULL;
    }

    int counter = 1;
    for (int i = 0; i < MAX_Y; i++)
        memcpy(copy[i], board[i], MAX_X);

    bool moved = true;
    while (moved) {
        moved = move(board, copy, directions[0]);
        for (int i = 0; i < MAX_Y; i++)
            memcpy(board[i], copy[i], MAX_X);
        moved = move(board, copy, directions[1]) || moved;
        for (int i = 0; i < MAX_Y; i++)
            memcpy(board[i], copy[i], MAX_X);
        printBoard(board);
        printf("%d \n", counter++);
    }
}
