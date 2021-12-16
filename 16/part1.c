#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

// decimal to binary
bool *decimalToBinary(int decimal) {
    bool *binary = malloc(4);
    int i = 3;
    while (decimal > 0) {
        binary[i] = decimal % 2;
        decimal /= 2;
        i--;
    }
    return binary;
}

int binaryToDecimal(bool *binary, int length) {
    int decimal = 0;
    int i = 0;
    while (i < length) {
        decimal += binary[i] * pow(2, length - i - 1);
        i++;
    }
    return decimal;
}

int versionSum = 0;

int parsePacket(bool *packet, int len) {
    printf("BEGIN PACKET\n");
    int index = 0;
    int packetVersion = binaryToDecimal(packet, 3);
    versionSum += packetVersion;
    index += 3;
    int packetType = binaryToDecimal(packet + index, 3);
    index += 3;
    printf("Packet version: %d\n", packetVersion);
    printf("Packet type: %d\n", packetType);
    if (packetType == 4) {
        // literal
        bool *literalBits = malloc(len - index);
        int literalLen = 0;
        bool keepReading = true;
        while (keepReading) {
            if (packet[index] == 0)
                keepReading = false;
            memcpy(literalBits + literalLen, packet + index + 1, 4);
            index += 5;
            literalLen += 4;
        }
        int literal = binaryToDecimal(literalBits, literalLen);
        printf("Literal: %d\n", literal);
    } else {
        // operator
        if (packet[index++]) {
            int subNum = binaryToDecimal(packet + index, 11);
            index += 11;
            printf("Subpacket length: %d\n", subNum);
            while (subNum > 0) {
                int subPacketLen = parsePacket(packet + index, 100);
                index += subPacketLen;
                subNum--;
            }
        } else {
            int subLen = binaryToDecimal(packet + index, 15);
            index += 15;
            printf("Subpacket length: %d\n", subLen);
            while (subLen > 0) {
                int subPacketLen = parsePacket(packet + index, 100);
                index += subPacketLen;
                subLen -= subPacketLen;
            }
        }
    }
    printf("END PACKET\n");
    return index;
}

int main() {
    FILE *fp = fopen(FILE_NAME, "r");

    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, fp);

    bool lineBits[strlen(line) * 4];
    int bitsLen = 0;
    int index = 0;
    while (line[index] != '\n') {
        char c = line[index];
        bool *binary = decimalToBinary(strtol(&c, NULL, 16));
        memcpy(lineBits + index * 4, binary, 4);
        bitsLen += 4;
        index++;
    }

    for (int i = 0; i < bitsLen; i++) {
        printf("%d", lineBits[i]);
    }
    printf("\n");

    int packetLen = parsePacket(lineBits, bitsLen);
    printf("Packet length: %d\n", packetLen);
    printf("Version sum: %d\n", versionSum);
}
