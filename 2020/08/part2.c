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

enum OP { acc, jmp, nop };

struct instruction {
    enum OP op;
    int arg;
};

void run(struct instruction *program, int program_len) {
    int reg = 0;
    bool visited[program_len];
    memset(visited, 0, sizeof(visited));
    for (int i = 0; i < program_len; i++) {
        if (visited[i]) {
            return;
        }
        visited[i] = true;
        switch (program[i].op) {
        case acc:
            reg += program[i].arg;
            break;
        case jmp:
            i += program[i].arg - 1;
            break;
        case nop:
            break;
        }
    }
    printf("last reg %d\n", reg);
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    struct instruction *program = malloc(sizeof(struct instruction) * 1000);
    struct instruction **swap = malloc(sizeof(struct instruction) * 1000);
    int program_len = 0;
    int swap_len = 0;
    while (getline(&line, &len, file) != EOF) {
        char *op = strsep(&line, " ");
        program[program_len].arg = atoi(strsep(&line, " "));

        switch (op[0]) {
        case 'n':
            program[program_len].op = nop;
            swap[swap_len++] = &program[program_len];
            break;
        case 'j':
            program[program_len].op = jmp;
            swap[swap_len++] = &program[program_len];
            break;
        case 'a':
            program[program_len].op = acc;
            break;
        default:
            assert(false);
        }
        program_len++;
    }

    for (int i = 0; i < swap_len; i++) {
        swap[i]->op = swap[i]->op == nop ? jmp : nop;
        run(program, program_len);
        swap[i]->op = swap[i]->op == nop ? jmp : nop;
    }
}
