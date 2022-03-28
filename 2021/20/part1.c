#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"
#define START_SIZE 220
#define START_POS 60

bool **board;
int board_size;
bool algorithm[513] = {0};
bool flick = false;

void printBoard(bool **b) {
    for (int i = 0; i < board_size; i++) {
        for (int j = 0; j < board_size; j++) {
            printf(b[i][j] ? "#" : ".");
        }
        printf("\n");
    }
}

bool calculate(int x, int y) {
    bool bin[9] = {0};
    int i = 0;
    for (int j = -1; j <= 1; j++) {
        for (int k = -1; k <= 1; k++) {
            int newX = x + k;
            int newY = y + j;
            if (newX < 0 || newX >= board_size || newY < 0 ||
                newY >= board_size) {
                bin[i] = flick ? algorithm[0] : algorithm[511];
            } else
                bin[i] = board[newY][newX];
            i++;
        }
    }
    // convert in to binary
    char str[9];
    for (i = 0; i < 9; i++) {
        if (bin[i]) {
            str[i] = '1';
        } else {
            str[i] = '0';
        }
    }
    return algorithm[strtol(str, NULL, 2)];
}

int countPixel(bool **b) {
    int count = 0;
    for (int i = 0; i < board_size; i++)
        for (int j = 0; j < board_size; j++)
            if (b[i][j])
                count++;
    return count;
}

int main() {
    board_size = START_SIZE;
    board = malloc(sizeof(bool *) * board_size);
    for (int i = 0; i < board_size; i++) {
        board[i] = malloc(sizeof(bool) * board_size);
        memset(board[i], 0, sizeof(bool) * board_size);
    }
    FILE *fs = fopen(FILE_NAME, "r");
    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, fs);
    for (int i = 0; line[i] != '\n'; i++) {
        if (line[i] == '#')
            algorithm[i] = true;
        else if (line[i] == '.')
            algorithm[i] = false;
        else
            assert(false);
    }
    getline(&line, &len, fs);
    int x = START_POS;
    int y = START_POS;
    while (getline(&line, &len, fs) != -1) {
        for (int i = 0; line[i] != '\n'; i++) {
            if (line[i] == '#')
                board[y][x] = true;
            else if (line[i] == '.')
                board[y][x] = false;
            else
                assert(false);
            x++;
        }
        x = START_POS;
        y++;
    }
    fclose(fs);

    bool **newBoard = malloc(sizeof(bool *) * (board_size));
    for (int i = 0; i < board_size; i++) {
        newBoard[i] = malloc(sizeof(bool) * (board_size));
        memset(newBoard[i], 0, sizeof(bool) * (board_size));
    }
    printBoard(board);
    printf("\n");

    for (int cycle = 0; cycle < 2; cycle++) {
        for (int i = 0; i < board_size; i++) {
            for (int j = 0; j < board_size; j++) {
                newBoard[i][j] = calculate(j, i);
            }
        }
        flick = !flick;
        for (int i = 0; i < board_size; i++)
            memcpy(board[i], newBoard[i], sizeof(bool) * board_size);
        printBoard(board);
        printf("\n");
    }
    printf("%d\n", countPixel(board));
}
