#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#define PREV 25
#else
#define FILE_NAME "test"
#define PREV 5
#endif

bool find_sum(long int *nums, int nums_len) {
    for (int i = nums_len - PREV; i < nums_len; i++) {
        for (int j = nums_len - PREV; j < nums_len; j++) {
            if (i == j)
                continue;
            if (nums[i] + nums[j] == nums[nums_len]) {
                return true;
            }
        }
    }
    return false;
}

void find_contiguous(long int *nums, int nums_len) {
    for (int i = 0; i < nums_len; i++) {
        int base = i;
        int len = 0;
        long int sum = 0;
        while (sum < nums[nums_len]) {
            sum += nums[base + len];
            len++;
        }
        if (sum == nums[nums_len]) {
            long int max = nums[base];
            long int min = nums[base];
            for (int i = base; i < base + len; i++) {
                if (nums[i] > max)
                    max = nums[i];
                if (nums[i] < min)
                    min = nums[i];
            }
            printf("sum=%ld min=%ld max=%ld weak=%ld\n", sum, min, max,
                   min + max);
            return;
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
    long int nums[1000];
    memset(nums, 0, sizeof(nums));
    int nums_len = 0;
    while (getline(&line, &len, file) != EOF) {
        nums[nums_len] = atoi(line);
        if (nums_len > PREV) {
            if (!find_sum(nums, nums_len))
                break;
        }
        nums_len++;
    }
    find_contiguous(nums, nums_len);
}
