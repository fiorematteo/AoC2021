#include <stdio.h>
#include <stdlib.h>

void readFile(int *input) {
    FILE *fp = fopen("input", "r");
    char *line = malloc(sizeof(char) * 10);
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
    int input[3000] = {0};
    readFile(input);
    int counter = 0;
    for (int i = 1; i <= 2500; i++){
        if (input[i - 1] < input[i])
            counter++;
    }
    printf("%d\n", counter);
    return 0;
}
