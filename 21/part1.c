#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

int die = 1;
int dieCount = 0;

int score1 = 0;
int score2 = 0;
int pos1 = 0;
int pos2 = 0;

int modulo(int a, int b) {
    while (a > b)
        a -= b;
    return a;
}

int roll() {
    printf(" %d+%d+%d ", die, modulo((die + 1), 100), modulo((die + 2), 100));
    int res = die + modulo((die + 1), 100) + modulo((die + 2), 100);
    die = modulo(die + 3, 100);
    dieCount+=3;
    return res;
}

int main() {
    FILE *fs = fopen(FILE_NAME, "r");
    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, fs);
    pos1 = line[strlen(line) - 2] - '0';
    getline(&line, &len, fs);
    pos2 = line[strlen(line) - 2] - '0';
    fclose(fs);

    while (true) {
        printf("Player 1 rolls");
        pos1 = modulo(pos1 + roll(), 10);
        score1 += pos1;
        printf("and moves to space %d for a total score of %d.\n", pos1,
               score1);
        if (score1 >= 1000)
            break;
        printf("Player 2 rolls");
        pos2 = modulo(pos2 + roll(), 10);
        score2 += pos2;
        printf("and moves to space %d for a total score of %d.\n", pos2,
               score2);
        if (score2 >= 1000)
            break;
    }
    printf("score 1: %d\n", score1);
    printf("score 2: %d\n", score2);
    printf("die count: %d\n", dieCount);
    printf("solution %d\n", (score1 > score2 ? score2 : score1) * dieCount);
}
