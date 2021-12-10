#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>


#define FILE_NAME "input"
//#define FILE_NAME "test"

#define STACK_SIZE 100000

enum state { TONDE, QUADRE, GRAFFE, DIAGONALI };
typedef enum state state;

struct stack {
    state array[STACK_SIZE];
    int top;
};
typedef struct stack stack;

state pop(stack *s) {
    if (s->top == -1) {
        printf("Stack is empty\n");
        exit(1);
    }
    return s->array[s->top--];
}

void push(stack *s, state x) {
    if (s->top == STACK_SIZE - 1) {
        printf("Stack is full\n");
        exit(1);
    }
    s->array[++s->top] = x;
}

void sortArray(int64_t arr[], int n) {
    int i, j;
    for (i = 0; i < n - 1; i++) {
        for (j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int64_t temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
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

    int64_t points[100];
    int pointsIndex = 0;

    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, file) != EOF) {
        stack stackState = {.top = -1};
        bool exit = false;

        while (*line != '\n' && !exit) {
            switch (*line) {
            case '(':
                push(&stackState, TONDE);
                break;
            case '[':
                push(&stackState, QUADRE);
                break;
            case '{':
                push(&stackState, GRAFFE);
                break;
            case '<':
                push(&stackState, DIAGONALI);
                break;
            case ')':
                if (pop(&stackState) != TONDE) {
                    exit = true;
                }
                break;
            case ']':
                if (pop(&stackState) != QUADRE) {
                    exit = true;
                }
                break;
            case '}':
                if (pop(&stackState) != GRAFFE) {
                    exit = true;
                }
                break;
            case '>':
                if (pop(&stackState) != DIAGONALI) {
                    exit = true;
                }
                break;
            }
            line++;
        }
        line = NULL;
        if (exit) {
            continue;
        }
        int64_t localPoints = 0;
        while (stackState.top != -1) {
            localPoints *= 5;
            switch (pop(&stackState)) {
            case TONDE:
                printf(")");
                localPoints += 1;
                break;
            case QUADRE:
                printf("]");
                localPoints += 2;
                break;
            case GRAFFE:
                printf("}");
                localPoints += 3;
                break;
            case DIAGONALI:
                printf(">");
                localPoints += 4;
                break;
            }
        }
        printf("  %lu\n", localPoints);
        points[pointsIndex++] = localPoints;
    }
    sortArray(points, pointsIndex);
    printf("%ld\n", points[pointsIndex / 2]);
    return 0;
}
