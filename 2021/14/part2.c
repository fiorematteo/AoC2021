#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

typedef struct pair pair;
struct pair {
    int64_t count;
    char chars[2];
    char left[2], right[2];
};

#define PAIR_COUNT 100
pair pairs[PAIR_COUNT];
int count = 0;
int64_t letters[26];

int findPair(char p[2]) {
    for (int i = 0; i < count; i++) {
        if (pairs[i].chars[0] == p[0] && pairs[i].chars[1] == p[1]) {
            return i;
        }
    }
    return count++;
}

int main() {
    char *polymer = NULL;
    polymer = malloc(INT_MAX);
    size_t len = 0;
    FILE *fp = fopen(FILE_NAME, "r");
    getline(&polymer, &len, fp);
    fgetc(fp);

    char a, b, c;
    while (fscanf(fp, "%c%c -> %c\n", &a, &b, &c) != EOF) {
        pairs[count].count = 0;
        pairs[count].chars[0] = a;
        pairs[count].chars[1] = b;
        pairs[count].left[0] = a;
        pairs[count].left[1] = c;
        pairs[count].right[0] = c;
        pairs[count].right[1] = b;
        count++;
    }
    fclose(fp);

    for (int i = 0; polymer[i + 2]; i++) {
        int index = findPair(&polymer[i]);
        if (index != -1) {
            pairs[index].count++;
        }
    }

    memset(letters, 0, sizeof(letters));
    for (int i = 0; polymer[i] != '\n'; i++) {
        letters[polymer[i] - 'A']++;
    }

    for (int k = 0; k < 40; k++) {
        pair newPairs[PAIR_COUNT];
        memcpy(newPairs, pairs, sizeof(pairs));
        for (int i = 0; i < count; i++) {
            int left = findPair(pairs[i].left);
            int right = findPair(pairs[i].right);
            newPairs[left].count += pairs[i].count;
            newPairs[right].count += pairs[i].count;
            newPairs[i].count -= pairs[i].count;
            letters[newPairs[left].chars[1] - 'A'] += pairs[i].count;
        }
        memcpy(pairs, newPairs, sizeof(pairs));
    }
    int64_t max = 0;
    int64_t min = LONG_MAX;
    for (int i = 0; i < 26; i++) {
        if (letters[i] > max) {
            max = letters[i];
        }
        if (letters[i] < min && letters[i] != 0) {
            min = letters[i];
        }
    }

    printf("max: %ld, min: %ld, result: %ld\n", max, min, max - min);
}
