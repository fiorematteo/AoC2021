#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

void sort(int *arr, int len) {
    int i, j, tmp;
    for (i = 0; i < len; i++) {
        for (j = i + 1; j < len; j++) {
            if (arr[i] > arr[j]) {
                tmp = arr[i];
                arr[i] = arr[j];
                arr[j] = tmp;
            }
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
    int nums[1000];
    memset(nums, 0, sizeof(nums));
    int nums_len = 0;
    while (getline(&line, &len, file) != EOF) {
        nums[nums_len++] = atoi(line);
    }
    sort(nums, nums_len);
    int a = 0, b = 1;
    if (nums[0] == 1)
        a++;
    else if (nums[0] == 3)
        b++;
    for (int i = 1; i < nums_len; i++) {
        if (nums[i] - nums[i - 1] == 1)
            a++;
        else if (nums[i] - nums[i - 1] == 3)
            b++;
        else if (nums[i] - nums[i - 1] > 3)
            exit(-1);
    }
    printf("result %d*%d=%d\n", a, b, a * b);
}
