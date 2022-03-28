#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
//#define PICKED_NUMBERS_SIZE 27
//#define BOARD_NUM 3
#define FILE_NAME "input"
#define PICKED_NUMBERS_SIZE 99
#define BOARD_NUM 100

typedef struct {
    int value;
    bool marked;
} number;

void parseLine(char *line, number *numbers) {
    int i = 0;
    char *token = strtok(line, " ");
    while (token != NULL) {
        numbers[i].value = atoi(token);
        numbers[i].marked = false;
        token = strtok(NULL, " ");
        i++;
    }
}

void printBoards(number ***boards) {
    for (int i = 0; i < BOARD_NUM; i++) {
        for (int j = 0; j < 5; j++) {
            for (int k = 0; k < 5; k++) {
                printf("%d ", boards[i][j][k].value);
            }
            printf("\n");
        }
        printf("\n");
    }
}

int sumUnmarked(number **numbers) {
    int sum = 0;
    for (int i = 0; i < 5; i++) {
        for (int j = 0; j < 5; j++) {
            if (!numbers[i][j].marked) {
                sum += numbers[i][j].value;
            }
        }
    }
    return sum;
}

bool checkWinner(number **board) {
    // check rows
    for (int i = 0; i < 5; i++) {
        int count = 0;
        for (int j = 0; j < 5; j++) {
            if (board[i][j].marked) {
                count++;
            }
        }
        if (count == 5) {
            return true;
        }
    }
    // check columns
    for (int i = 0; i < 5; i++) {
        int count = 0;
        for (int j = 0; j < 5; j++) {
            if (board[j][i].marked) {
                count++;
            }
        }
        if (count == 5) {
            return true;
        }
    }
    return false;
}

int main() {
    FILE *f = fopen(FILE_NAME, "r");

    int picked_numbers[PICKED_NUMBERS_SIZE];
    char *num = NULL;
    size_t len = 0;
    int index = 0;
    while (getdelim(&num, &len, ',', f) != -1) {
        picked_numbers[index] = atoi(num);
        index++;
    }
    for (int i = 0; i < PICKED_NUMBERS_SIZE; i++) {
        printf("%d\n", picked_numbers[i]);
    }

    number ***boards = malloc(1000);
    for (int i = 0; i < 1000; i++) {
        boards[i] = malloc(100);
        for (int j = 0; j < 100; j++) {
            boards[i][j] = malloc(100);
        }
    }

    int j = 0;
    int i = 0;
    size_t size = 5;
    char *line = malloc(size);

    fseek(f, 0, SEEK_SET);
    getline(&line, &size, f);
    getline(&line, &size, f);

    while (getline(&line, &size, f) != EOF) {
        if (line[0] == '\n') {
            i++;
            j = 0;
            continue;
        }
        parseLine(line, boards[i][j]);
        j++;
    }

    printBoards(boards);
    // loop over picked numbers
    for (i = 0; i < PICKED_NUMBERS_SIZE; i++) {
        // loop over boards
        for (j = 0; j < BOARD_NUM; j++) {
            // loop over rows
            for (int k = 0; k < 5; k++) {
                // loop over columns
                for (int l = 0; l < 5; l++) {
                    if (boards[j][k][l].value == picked_numbers[i]) {
                        boards[j][k][l].marked = true;
                    }
                }
            }
            if (checkWinner(boards[j])){
                printf("last number: %d\n", picked_numbers[i]);
                printf("sum: %d\n", sumUnmarked(boards[j]));
                printf("result: %d\n", picked_numbers[i] * sumUnmarked(boards[j]));
                return 0;
            }
        }
    }
    return 0;
}
