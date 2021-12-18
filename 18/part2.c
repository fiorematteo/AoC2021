#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

typedef struct num num;

struct num {
    int n;
    int depth;
};

void delete (num *n, int *len, int i) {
    memmove(n + i, n + i + 1, (*len - i - 1) * sizeof(num));
    (*len)--;
}

void insert(num *n, int *len, int i, num value) {
    memmove(n + i + 1, n + i, (*len - i) * sizeof(num));
    n[i] = value;
    (*len)++;
}

void printNums1(num *nums, int numLen) {
    for (int i = 0; i < numLen; i++) {
        printf("%d-%d|", nums[i].n, nums[i].depth);
    }
    printf("\n");
}

void printNums2(num *nums, int numLen) {
    int d = 1;
    for (int i = 0; i < numLen; i++) {
        while (d != nums[i].depth) {
            if (d < nums[i].depth) {
                printf("[");
                d++;
            } else {
                printf("]");
                d--;
            }
        }
        printf("%d,", nums[i].n);
    }
    while (d > 0) {
        printf("]");
        d--;
    }
    printf("\n");
}

void (*printNums)(num *, int) = printNums2;

int magnitude(num *nums, int numLen) {
    while (numLen > 1) {
        for (int i = 0; i < numLen; i++) {
            if (nums[i].depth == nums[i + 1].depth) {
                nums[i].n = 3 * nums[i].n + 2 * nums[i + 1].n;
                nums[i].depth--;
                delete (nums, &numLen, i + 1);
                break;
            }
        }
    }
    return nums[0].n;
}

int reduce(num *nums, int numLen) {
    printNums(nums, numLen);
    bool updated = true;
    while (updated) {
        updated = false;
        for (int i = 0; i < numLen; i++) {
            // explode
            if (nums[i].depth > 4 && nums[i + 1].depth == nums[i].depth) {
                if (i != 0)
                    nums[i - 1].n += nums[i].n;
                nums[i].n = 0;
                nums[i].depth--;
                nums[i].n = 0;
                i++;
                if (i < numLen)
                    nums[i + 1].n += nums[i].n;
                nums[i].n = 0;
                delete (nums, &numLen, i);
                updated = true;
                break;
            }
        }
        if (updated)
            continue;
        for (int i = 0; i < numLen; i++) {
            if (nums[i].n >= 10) {
                bool dispari = true;
                if (nums[i].n % 2 == 0)
                    dispari = false;
                nums[i].n /= 2;
                nums[i].depth += 1;
                insert(nums, &numLen, i + 1,
                       (num){nums[i].n + dispari, nums[i].depth});
                updated = true;
                break;
            }
        }
    }
    return numLen;
}

num *add(num *a, int aLen, num *b, int bLen) {
    num *c = malloc(sizeof(num) * (aLen + bLen) + 100);
    int cLen = 0;
    for (int i = 0; i < aLen; i++) {
        a[i].depth++;
        c[cLen++] = a[i];
    }
    for (int i = 0; i < bLen; i++) {
        b[i].depth++;
        c[cLen++] = b[i];
    }
    free(a);
    free(b);
    return c;
}

num *copy(num *nums, int numLen) {
    num *c = malloc(sizeof(num) * numLen);
    memcpy(c, nums, numLen * sizeof(num));
    return c;
}

int main() {
    num nums[100][1000];
    int numLen = 0;
    int lenght[100];

    FILE *fs = fopen(FILE_NAME, "r");
    char *line = NULL;
    size_t len = 0;
    int depth = 0;
    int lineIndex = 0;
    while (getline(&line, &len, fs) != EOF) {
        for (int i = 0; line[i] && line[i] != '\n'; i++) {
            if (line[i] == '[') {
                depth++;
            } else if (line[i] == ']') {
                depth--;
            } else if (line[i] == ',') {
                continue;
            } else {
                nums[lineIndex][numLen].n = atoi(&line[i]);
                nums[lineIndex][numLen].depth = depth;
                numLen++;
            }
        }
        line = NULL;
        lenght[lineIndex] = numLen;
        numLen = 0;
        lineIndex++;
    }

    int maxMag = 0;
    for (int i = 0; i < lineIndex; i++) {
        for (int j = 0; j < lineIndex; j++) {
            if (i == j)
                continue;
            num *addedNums = add(copy(nums[i], lenght[i]), lenght[i],
                                 copy(nums[j], lenght[j]), lenght[j]);
            numLen = reduce(addedNums, lenght[i] + lenght[j]);
            int result = magnitude(addedNums, numLen);
            printf("%d\n", result);
            if (result > maxMag)
                maxMag = result;
        }
    }
    printf("MAX: %d\n", maxMag);
}
