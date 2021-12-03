#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#define NUM_LEN 12
#define FILE_NAME "input"

void printBits(int n){
    int i;
    for(i = 0; i < NUM_LEN; i++){
        if(n & (1 << i)){
            printf("1");
        }
        else{
            printf("0");
        }
    }
    printf("\n");
}

int main() {
    int bits[NUM_LEN] = {0};
    int lines = 0;
    char bit = 0;
    int index = 0;

    FILE *f = fopen(FILE_NAME, "r");
    while ((bit = fgetc(f)) != EOF) {
        if (bit == '\n') {
            lines++;
            index = 0;
        } else {
            bits[index] += bit - '0';
            index++;
        }
    }
    //print bits
    printf("BITS: ");
    for (int i = 0; i < NUM_LEN; i++) {
        printf("%d, ", bits[i]);
    }
    printf("\n");

    // bits to int
    unsigned int gamma = 0;
    for (int i = 0; i < NUM_LEN; i++) {
        if (bits[i] < lines / 2)
            gamma += (int)pow(2, NUM_LEN - 1 - i);
    }

    unsigned int epsilon = 0;
    for (int i = 0; i < NUM_LEN; i++) {
        if (bits[i] > lines / 2)
            epsilon += (int)pow(2, NUM_LEN - 1 - i);
    }

    printf("gamma: %u\n", gamma);
    printBits(gamma);
    printf("\nepsilon: %u\n", epsilon);
    printBits(epsilon);
    printf("\nsolution: %u\n", gamma * epsilon);
    fclose(f);
}
