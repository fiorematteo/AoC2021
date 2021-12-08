#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

enum segments { a, b, c, d, e, f, g };

char enumToChar(enum segments seg) {
    switch (seg) {
    case a:
        return 'a';
    case b:
        return 'b';
    case c:
        return 'c';
    case d:
        return 'd';
    case e:
        return 'e';
    case f:
        return 'f';
    case g:
        return 'g';
    }
    return 'A';
}

typedef bool digit[7];

void sort_string(char *str, bool digitValue[7]) {
    for (int i = 0; str[i]; i++) {
        if (str[i] == '\n')
            break;
        digitValue[str[i] - 'a'] = true;
    }
}

int countSegments(digit dig) {
    int count = 0;
    for (int i = 0; i < 7; i++) {
        if (dig[i]) {
            count++;
        }
    }
    return count;
}

#define check4(A, B, C, D) (check2(A, B) && check2(C, D))
#define check3(A, B, C) (check2(A, B) && dig[wiring[C]])
#define check2(A, B) (dig[wiring[A]] && dig[wiring[B]])

int decodeDigit(digit dig, int wiring[7]) {
    if (check4(a, b, c, d) && check3(e, f, g))
        return 8;
    if (check4(a, b, f, g) && countSegments(dig) == 6) {
        if (check2(c, d))
            return 9;
        if (check2(d, e))
            return 6;
        if (check2(c, e))
            return 0;
    }
    if (check3(a, d, g) && countSegments(dig) == 5) {
        if (check2(c, e))
            return 2;
        if (check2(c, f))
            return 3;
        if (check2(b, f))
            return 5;
    }
    if (check4(b, c, d, f) && countSegments(dig) == 4)
        return 4;
    if (check3(a, c, f) && countSegments(dig) == 3)
        return 7;
    if (check2(c, f) && countSegments(dig) == 2)
        return 1;
    return -1;
}

bool validWiring(digit digits[10], int wiring[7]) {
    for (int i = 0; i < 10; i++) {
        if (decodeDigit(digits[i], wiring) == -1) {
            return false;
        }
    }
    return true;
}

void solveWiring(digit digits[10], int wiring[7]) {
    for (int pos_a = 0; pos_a < 7; pos_a++)
        for (int pos_b = 0; pos_b < 7; pos_b++)
            if (pos_b != pos_a)
                for (int pos_c = 0; pos_c < 7; pos_c++)
                    if (pos_c != pos_b && pos_c != pos_a)
                        for (int pos_d = 0; pos_d < 7; pos_d++)
                            if (pos_d != pos_a && pos_d != pos_b &&
                                pos_d != pos_c)
                                for (int pos_e = 0; pos_e < 7; pos_e++)
                                    if (pos_e != pos_a && pos_e != pos_b &&
                                        pos_e != pos_c && pos_e != pos_d)
                                        for (int pos_f = 0; pos_f < 7; pos_f++)
                                            if (pos_f != pos_a &&
                                                pos_f != pos_b &&
                                                pos_f != pos_c &&
                                                pos_f != pos_d &&
                                                pos_f != pos_e)
                                                for (int pos_g = 0; pos_g < 7;
                                                     pos_g++)
                                                    if (pos_g != pos_a &&
                                                        pos_g != pos_b &&
                                                        pos_g != pos_c &&
                                                        pos_g != pos_d &&
                                                        pos_g != pos_e &&
                                                        pos_g != pos_f) {
                                                        wiring[a] = pos_a;
                                                        wiring[b] = pos_b;
                                                        wiring[c] = pos_c;
                                                        wiring[d] = pos_d;
                                                        wiring[e] = pos_e;
                                                        wiring[f] = pos_f;
                                                        wiring[g] = pos_g;
                                                        if (validWiring(digits,
                                                                        wiring))
                                                            return;
                                                    }
    assert(false);
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    int sum = 0;

    char *input = malloc(sizeof(char) * 200);
    size_t len = 1000;
    while (getline(&input, &len, file) != -1) {

        digit digits[10];
        for (int i = 0; i < 10; i++) {
            for (int j = 0; j < 7; j++) {
                digits[i][j] = false;
            }
        }
        digit display[4];
        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 7; j++) {
                display[i][j] = false;
            }
        }

        char *token = strtok(input, " ");
        for (int i = 0; i < 15; i++) {
            if (i < 10) {
                sort_string(token, digits[i]);
            } else if (i > 10) {
                sort_string(token, display[i - 11]);
            }
            token = strtok(NULL, " ");
        }

        int wiring[7] = {a, b, c, d, e, f, g};
        solveWiring(digits, wiring);

        for (int i = 0; i < 4; i++) {
            sum += decodeDigit(display[i], wiring) * pow(10, 3 - i);
        }
    }
    printf("%d\n", sum);
}
