#include <stdio.h>
#include <stdlib.h>

#define arraySize(a) (sizeof(a) / sizeof(a[0]))

void readFile(int *input) {
    FILE *fp = fopen("input", "r");
    char *line = malloc(sizeof(char) * 100);
    size_t len = 0;
    ssize_t read;
    int i = 0;
    while ((read = getline(&line, &len, fp)) != -1) {
        input[i] = atoi(line);
        i++;
    }
    fclose(fp);
}

int main() {
    int input[3000];
    readFile(input);
    int arrayOfSums[3000] = {0};
    for (size_t i = 2; i < arraySize(input); i++) {
        arrayOfSums[i] = input[i] + input[i - 1] + input[i - 2];
    }
    int counter = -1;
    for (size_t i = 1; i < arraySize(arrayOfSums); i++) {
        if (arrayOfSums[i - 1] < arrayOfSums[i])
            counter++;
    }
    printf("%d\n", counter);
    return 0;
}
