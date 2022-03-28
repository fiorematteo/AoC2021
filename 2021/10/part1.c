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

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    int points = 0;
    stack stackState = {.top = -1};

    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, file) != EOF) {
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
                    printf("carattere illegale )\n");
                    points += 3;
                    exit = true;
                }
                break;
            case ']':
                if (pop(&stackState) != QUADRE) {
                    printf("carattere illegale ]\n");
                    points += 57;
                    exit = true;
                }
                break;
            case '}':
                if (pop(&stackState) != GRAFFE) {
                    printf("carattere illegale }\n");
                    points += 1197;
                    exit = true;
                }
                break;
            case '>':
                if (pop(&stackState) != DIAGONALI) {
                    printf("carattere illegale >\n");
                    points += 25137;
                    exit = true;
                }
                break;
            }
            line++;
        }
        line = NULL;
    }
    printf("%d\n", points);

    return 0;
}
