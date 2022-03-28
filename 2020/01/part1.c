#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_NAME "input"
//#define FILE_NAME "test"

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    int n;
    int numbers[1000];
    int index = 0;
    memset(numbers, 0, sizeof(numbers));
    while (fscanf(file, "%d", &n) != EOF) {
        numbers[index++] = n;
    }

    for (int i = 0; i < 1000; i++) {
        for (int j = 0; j < 1000; j++) {
            printf("%d + %d = %d\n", numbers[i], numbers[j], numbers[i] + numbers[j]);
            if (numbers[i] + numbers[j] == 2020) {
                printf("%d\n", numbers[i] * numbers[j]);
                return 0;
            }
        }
    }
}
