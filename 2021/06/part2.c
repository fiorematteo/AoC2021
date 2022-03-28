#include <stdio.h>
#include <stdlib.h>

#define FILE_NAME "input"
//#define FILE_NAME "test"
#define DAYS 257

int main() {
    // read input file
    FILE *f = fopen(FILE_NAME, "r");

    int64_t fishes[9] = {0};

    char c;
    while ((c = fgetc(f)) != EOF) {
        if (c == ',' || c == ' ' || c == '\n') {
            continue;
        }
        fishes[c - '0' + 1]++;
    }

    int days = 0;
    while (days < DAYS) {
        int i;
        int64_t tmp = fishes[0];
        for (i = 0; i < 8; i++)
            fishes[i] = fishes[i + 1];
        fishes[8] = tmp;
        fishes[6] += tmp;
        days++;
    }
    // count all fishses
    int64_t sum = 0;
    for (int i = 0; i < 9; i++) {
        sum += fishes[i];
    }
    printf("%ld\n", sum);
}
