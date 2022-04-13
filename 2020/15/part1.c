#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 0
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    int nums[2020];
    memset(nums, -1, sizeof(nums));
    int nums_len = 0;
    char *token;
    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, file);
    line[strlen(line) - 1] = '\0';
    while ((token = strsep(&line, ",")) != NULL) {
        nums[nums_len++] = atoi(token);
        printf("%d\n", nums[nums_len - 1]);
    }

    while (nums_len < 2020) {
        bool to_add = true;
        for (int i = nums_len - 2; i >= 0; i--) {
            if (nums[i] == nums[nums_len - 1]) {
                nums[nums_len] = nums_len - i - 1;
                nums_len++;
                to_add = false;
                break;
            }
        }
        if (to_add)
            nums[nums_len++] = 0;
        printf("%d\n", nums[nums_len - 1]);
    }
}
