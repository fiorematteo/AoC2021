#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NUM_LEN 12
#define FILE_NAME "input"
#define LINES 1000

// display 2d array of bools
void display_array(bool **array, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        if (array[i] == NULL)
            continue;
        for (int j = 0; j < cols; j++) {
            printf("%d", array[i][j]);
        }
        printf("\n");
    }
}

void deepBoolCopy(bool *dest, bool *src, int size) {
    for (int i = 0; i < size; i++) {
        dest[i] = src[i];
    }
}

void countBit(bool **arr, int index, int *tc, int *fc) {
    for (int i = 0; i < LINES; i++) {
        if (arr[i] == NULL)
            continue;
        if (arr[i][index])
            (*tc)++;
        else
            (*fc)++;
    }
}

int extractValue(bool **arr) {
    bool *temp = malloc(sizeof(bool) * NUM_LEN);
    for (int i = 0; i < LINES; i++) {
        if (arr[i] == NULL)
            continue;
        deepBoolCopy(temp, arr[i], NUM_LEN);
    }
    // temp to int firt bit is most significant
    int value = 0;
    for (int i = 0; i < NUM_LEN; i++) {
        value += temp[i] * pow(2, (NUM_LEN - 1 - i));
    }
    free(temp);
    return value;
}

int main() {
    bool **valueBits = malloc(sizeof(bool) * 10000);
    for (int i = 0; i < LINES; i++) {
        valueBits[i] = malloc(sizeof(bool) * NUM_LEN);
    }
    int index = 0;

    // Read file
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file\n");
        return 1;
    }
    char *line = malloc(sizeof(char) * NUM_LEN);
    while ((line = fgets(line, NUM_LEN + 1, file)) != NULL) {
        if (line[0] == '\n')
            continue;
        for (int i = 0; i < NUM_LEN; i++) {
            valueBits[index][i] = (line[i] == '1');
        }
        index++;
    }

    // deep copy of valueBits
    bool **valueBitsCopy = malloc(sizeof(bool) * 10000);
    for (int i = 0; i < LINES; i++) {
        valueBitsCopy[i] = malloc(sizeof(bool) * NUM_LEN);
    }
    for (int i = 0; i < LINES; i++) {
        deepBoolCopy(valueBitsCopy[i], valueBits[i], NUM_LEN);
    }

    // oxygen
    int bitIndex = 0;
    int removed = 0;
    while (removed < LINES - 1 && bitIndex < NUM_LEN) {
        int trueCount = 0, falseCount = 0;
        countBit(valueBits, bitIndex, &trueCount, &falseCount);
        bool mostCommon = (trueCount >= falseCount);
        for (int i = 0; i < LINES; i++) {
            if (valueBits[i] && removed < LINES - 1 &&
                mostCommon != valueBits[i][bitIndex]) {
                valueBits[i] = NULL;
                removed++;
                bitIndex = -1;
                break;
            }
        }
        bitIndex++;
    }
    display_array(valueBits, LINES, NUM_LEN);
    int oxygen = extractValue(valueBits);

    free(valueBits);
    valueBits = valueBitsCopy;
    // CO2
    bitIndex = 0;
    removed = 0;
    while (removed < LINES - 1 && bitIndex < NUM_LEN) {
        int trueCount = 0, falseCount = 0;
        countBit(valueBits, bitIndex, &trueCount, &falseCount);
        bool mostCommon = (trueCount < falseCount);
        for (int i = 0; i < LINES; i++) {
            if (valueBits[i] && removed < LINES - 1 &&
                mostCommon != valueBits[i][bitIndex]) {
                valueBits[i] = NULL;
                removed++;
            }
        }
        bitIndex++;
    }
    display_array(valueBits, LINES, NUM_LEN);
    int CO2 = extractValue(valueBits);

    printf("oxygen: %d\n", oxygen);
    printf("CO2: %d\n", CO2);
    printf("risultato: %d\n", oxygen * CO2);
    return 0;
}
