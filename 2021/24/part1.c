#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

typedef enum opcode opcode;
enum opcode { INP, ADD, MUL, DIV, MOD, EQL };

typedef struct instruction instruction;
struct instruction {
    opcode opcode;
    long *a;
    long *b;
    long aImm;
    long bImm;
};

long w = 0, x = 0, y = 0, z = 0;

void printRegisters() {
    printf("w: %ld, x: %ld, y: %ld, z: %ld\n", w, x, y, z);
}

long *stringToPtr(char a) {
    switch (a) {
    case 'w':
        return &w;
    case 'x':
        return &x;
    case 'y':
        return &y;
    case 'z':
        return &z;
    default:
        assert(false);
    }
}

bool isLetter(char a) { return a >= 'a' && a <= 'z'; }

void printOP(int op) {
    switch (op) {
    case INP:
        printf("INP: ");
        break;
    case ADD:
        printf("ADD: ");
        break;
    case MUL:
        printf("MUL: ");
        break;
    case DIV:
        printf("DIV: ");
        break;
    case MOD:
        printf("MOD: ");
        break;
    case EQL:
        printf("EQL: ");
        break;
    }
}
void printInstruction(instruction ints) {
    printOP(ints.opcode);
    printf("%ld %ld\n", *ints.a, *ints.b);
}

bool simulate(instruction *program, int programLen, char *input) {
    int pc = 0;
    int inputCounter = 0;
    while (pc < programLen) {
        switch (program[pc].opcode) {
        case INP:
            printRegisters();
            *program[pc].a = input[inputCounter++] - '0';
            break;
        case ADD:
            *program[pc].a = *program[pc].a + *program[pc].b;
            break;
        case MUL:
            *program[pc].a = *program[pc].a * *program[pc].b;
            break;
        case DIV:
            if (*program[pc].b == 0)
                return false;
            *program[pc].a = *program[pc].a / *program[pc].b;
            break;
        case MOD:
            if (*program[pc].a < 0)
                return false;
            if (*program[pc].b <= 0)
                return false;
            *program[pc].a = *program[pc].a % *program[pc].b;
            break;
        case EQL:
            *program[pc].a = *program[pc].a == *program[pc].b;
            break;
        }
        // printInstruction(program[pc]);
        // printRegisters();
        pc++;
    }
    if (z == 0)
        return true;
    return false;
}

int main() {
    FILE *fs = fopen(FILE_NAME, "r");
    assert(fs);
    instruction program[300];
    int programLen = 0;

    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, fs) != EOF) {
        if (line[0] == '\n')
            continue;
        char tmp1, tmp2;
        switch (line[0]) {
        case 'i':
            program[programLen].opcode = INP;
            sscanf(line, "inp %c", &tmp1);
            sscanf(line, "inp %ld", &program[programLen].aImm);
            if (isLetter(tmp1))
                program[programLen].a = stringToPtr(tmp1);
            program[programLen].bImm = 0;
            program[programLen].b = &program[programLen].bImm;
            break;
        case 'a':
            program[programLen].opcode = ADD;
            sscanf(line, "add %c %c", &tmp1, &tmp2);
            sscanf(line, "add %c %ld", &tmp1, &program[programLen].bImm);
            sscanf(line, "add %ld %c", &program[programLen].aImm, &tmp2);
            sscanf(line, "add %ld %ld", &program[programLen].aImm,
                   &program[programLen].bImm);
            if (isLetter(tmp1))
                program[programLen].a = stringToPtr(tmp1);
            else
                program[programLen].a = &program[programLen].aImm;
            if (isLetter(tmp2))
                program[programLen].b = stringToPtr(tmp2);
            else
                program[programLen].b = &program[programLen].bImm;
            break;
        case 'm':
            if (line[1] == 'u') {
                program[programLen].opcode = MUL;
                sscanf(line, "mul %c %c", &tmp1, &tmp2);
                sscanf(line, "mul %c %ld", &tmp1, &program[programLen].bImm);
                sscanf(line, "mul %ld %c", &program[programLen].aImm, &tmp2);
                sscanf(line, "mul %ld %ld", &program[programLen].aImm,
                       &program[programLen].bImm);
            } else {
                program[programLen].opcode = MOD;
                sscanf(line, "mod %c %c", &tmp1, &tmp2);
                sscanf(line, "mod %c %ld", &tmp1, &program[programLen].bImm);
                sscanf(line, "mod %ld %c", &program[programLen].aImm, &tmp2);
                sscanf(line, "mod %ld %ld", &program[programLen].aImm,
                       &program[programLen].bImm);
            }
            if (isLetter(tmp1))
                program[programLen].a = stringToPtr(tmp1);
            else
                program[programLen].a = &program[programLen].aImm;
            if (isLetter(tmp2))
                program[programLen].b = stringToPtr(tmp2);
            else
                program[programLen].b = &program[programLen].bImm;
            break;
        case 'd':
            program[programLen].opcode = DIV;
            sscanf(line, "div %c %c", &tmp1, &tmp2);
            sscanf(line, "div %c %ld", &tmp1, &program[programLen].bImm);
            sscanf(line, "div %ld %c", &program[programLen].aImm, &tmp2);
            sscanf(line, "div %ld %ld", &program[programLen].aImm,
                   &program[programLen].bImm);
            if (isLetter(tmp1))
                program[programLen].a = stringToPtr(tmp1);
            else
                program[programLen].a = &program[programLen].aImm;
            if (isLetter(tmp2))
                program[programLen].b = stringToPtr(tmp2);
            else
                program[programLen].b = &program[programLen].bImm;
            break;
        case 'e':
            program[programLen].opcode = EQL;
            sscanf(line, "eql %c %c", &tmp1, &tmp2);
            sscanf(line, "eql %c %ld", &tmp1, &program[programLen].bImm);
            sscanf(line, "eql %ld %c", &program[programLen].aImm, &tmp2);
            sscanf(line, "eql %ld %ld", &program[programLen].aImm,
                   &program[programLen].bImm);
            if (isLetter(tmp1))
                program[programLen].a = stringToPtr(tmp1);
            else
                program[programLen].a = &program[programLen].aImm;
            if (isLetter(tmp2))
                program[programLen].b = stringToPtr(tmp2);
            else
                program[programLen].b = &program[programLen].bImm;
            break;
        }
        programLen++;
    }

    long input = 99394899891971;
    char *inputStr = (char *)malloc(sizeof(char) * 14);
    bool result;
    while (true) {
        sprintf(inputStr, "%ld", input);
        bool skip = false;
        for (int i = 0; inputStr[i] != '\0'; i++)
            if (inputStr[i] == '0')
                skip = true;
        if (skip) {
            input--;
            continue;
        }
        result = simulate(program, programLen, inputStr);
        // printf("in: %ld, out: %d\n", input, result);
        if (result || input == 11111111111111)
            break;
        input--;
    }
    printf("in: %ld out %d\n", input, result);
}
