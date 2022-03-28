#include <stdio.h>
#include <stdlib.h>

int main() {
    int value;
    char *command = malloc(sizeof(char) * 50);
    int horizontal = 0;
    int depth = 0;
    FILE *f = fopen("input", "r");
    while (fscanf(f, "%s %d", command, &value) != EOF) {
        switch (command[0]) {
        case 'u':
            depth -= value;
            break;
        case 'd':
            depth += value;
            break;
        case 'f':
            horizontal += value;
            break;
        default:
            printf("error");
            exit(1);
        }
    }
    printf("horizontal = %d\n", horizontal);
    printf("depth = %d\n", depth);
    printf("mult = %d\n", horizontal * depth);
    return 0;
}
