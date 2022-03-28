#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

typedef struct match match;
struct match {
    int strIndex;
    char c;
    char *pair;
};

int findPair(char *str, char *pair) {
    for (int i = 0; str[i + 1]; i++) {
        if (pair[0] == str[i] && pair[1] == str[i + 1]) {
            return i;
        }
    }
    return -1;
}

char *insertChar(char *str, match m) {
    if (m.strIndex < 0) {
        return str;
    }
    m.strIndex++;
    char *newStr = malloc(50000);
    memset(newStr, '\0', 50000);
    strncpy(newStr, str, m.strIndex);
    newStr[m.strIndex] = m.c;
    strncpy(newStr + m.strIndex + 1, str + m.strIndex,
            strlen(str) - m.strIndex);
    newStr[strlen(str) + 1] = '\0';
    return newStr;
}

int countChar(char *str, char c) {
    int count = 0;
    for (int i = 0; str[i]; i++) {
        if (str[i] == c) {
            count++;
        }
    }
    return count;
}

int main() {
    char *pairs[500];
    for (int i = 0; i < 500; i++) {
        pairs[i] = malloc(3);
    }
    char chars[500];
    int count = 0;

    char *polymer = NULL;
    polymer = malloc(5000);
    size_t len = 0;
    FILE *fp = fopen(FILE_NAME, "r");
    getline(&polymer, &len, fp);
    for (int i = 0; polymer[i]; i++) {
        if (polymer[i] == '\n')
            polymer[i] = '\0';
    }
    while (fscanf(fp, "%s -> %c", pairs[count], &chars[count]) != EOF) {
        pairs[count][2] = '\0';
        count++;
    }
    fclose(fp);

    for (int k = 0; k < 10; k++) {
        match matches[count * sizeof(match) * 20];
        int matchCount = 0;

        for (int i = 0; i < count; i++) {
            for (int j = 0; polymer[j + 1]; j++) {
                if (polymer[j] == pairs[i][0] &&
                    polymer[j + 1] == pairs[i][1]) {
                    matches[matchCount].strIndex = j;
                    matches[matchCount].c = chars[i];
                    matches[matchCount].pair = pairs[i];
                    matchCount++;
                }
            }
        }

        bool inserted[matchCount];
        int boolCount = 0;
        memset(inserted, false, sizeof(inserted));

        while (boolCount < matchCount) {
            match maxMatch = {-100, '\0', NULL};
            int arrIndex = 0;
            for (int i = 0; i < matchCount; i++) {
                if (matches[i].strIndex == -1) {
                    inserted[i] = true;
                } else if (!inserted[i] &&
                           matches[i].strIndex > maxMatch.strIndex) {
                    maxMatch = matches[i];
                    arrIndex = i;
                }
            }
            inserted[arrIndex] = true;
            boolCount++;
            if (maxMatch.strIndex >= 0 && maxMatch.strIndex >= 0) {
                polymer = insertChar(polymer, maxMatch);
            }
        }
    }
    int letters[26];
    memset(letters, 0, sizeof(letters));
    for (int i = 0; i < 26; i++) {
        letters[i] = countChar(polymer, i + 'A');
    }
    int min = INT_MAX;
    int max = INT_MIN;
    for (int i = 0; i < 26; i++) {
        if (letters[i] < min && letters[i] != 0) {
            min = letters[i];
        }
        if (letters[i] > max) {
            max = letters[i];
        }
    }
    printf("max: %d, min: %d, solution: %d\n", max, min, max - min);
}
