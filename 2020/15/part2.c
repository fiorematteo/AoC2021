#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif
#define NUMS_LEN 30000000

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    int *nums;
    nums = malloc(NUMS_LEN * sizeof(int));
    memset(nums, -1, NUMS_LEN * sizeof(int));

    long turn = 1;
    char *value;
    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, file);
    line[strlen(line) - 1] = '\0';
    while ((value = strsep(&line, ",")) != NULL) {
        int v = atoi(value);
        nums[v] = turn;
        // printf("turn: %ld value: %d\n", turn, v);
        printf("%d\n", v);
        turn++;
    }

    int target = 30000000;
    int current_num = 0;
    while (turn < target) {
        if (turn % NUMS_LEN == 0)
            printf("turn: %ld value: %d\n", turn, current_num);
        int index = current_num;
        if (nums[index] == -1) {
            nums[index] = turn;
            current_num = 0;
        } else {
            current_num = turn - nums[index];
            nums[index] = turn;
        }
        turn++;
    }
    printf("turn: %ld value: %d\n", turn, current_num);
}
