#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

// decimal to binary
bool *decimalToBinary(int64_t decimal) {
    bool *binary = malloc(4);
    int64_t i = 3;
    while (decimal > 0) {
        binary[i] = decimal % 2;
        decimal /= 2;
        i--;
    }
    return binary;
}

int64_t binaryToDecimal(bool *binary, int64_t length) {
    int64_t decimal = 0;
    int64_t i = 0;
    while (i < length) {
        decimal += binary[i] * pow(2, length - i - 1);
        i++;
    }
    return decimal;
}

typedef struct {
    int64_t value;
    int64_t length;
} subPacket;

int64_t sum(subPacket *packet, int64_t length) {
    int64_t sum = 0;
    for (int64_t i = 0; i < length; i++)
        sum += packet[i].value;
    return sum;
}

int64_t product(subPacket *packet, int64_t length) {
    int64_t product = 1;
    for (int64_t i = 0; i < length; i++)
        product *= packet[i].value;
    return product;
}

int64_t min(subPacket *packet, int64_t length) {
    int64_t min = packet[0].value;
    for (int64_t i = 1; i < length; i++)
        if (packet[i].value < min)
            min = packet[i].value;
    return min;
}

int64_t max(subPacket *packet, int64_t length) {
    int64_t max = packet[0].value;
    for (int64_t i = 1; i < length; i++)
        if (packet[i].value > max)
            max = packet[i].value;
    return max;
}

bool greaterThan(subPacket *packet, int64_t length) {
    assert(length == 2);
    return packet[0].value > packet[1].value;
}

bool lessThan(subPacket *packet, int64_t length) {
    assert(length == 2);
    return packet[0].value < packet[1].value;
}

bool equalTo(subPacket *packet, int64_t length) {
    assert(length == 2);
    return packet[0].value == packet[1].value;
}

subPacket parsePacket(bool *packet) {
    printf("BEGIN PACKET\n");
    int64_t index = 0;
    // int64_t packetVersion = binaryToDecimal(packet, 3);
    index += 3;
    int64_t packetType = binaryToDecimal(packet + index, 3);
    index += 3;
    // printf("Packet version: %d\n", packetVersion);
    printf("Packet type: %ld\n", packetType);
    int64_t literal = -1;
    int64_t result = -1;
    bool isLiteral = false;
    if (packetType == 4) {
        isLiteral = true;
        // literal
        bool *literalBits = malloc(10000);
        int64_t literalLen = 0;
        bool keepReading = true;
        while (keepReading) {
            if (packet[index] == 0)
                keepReading = false;
            memcpy(literalBits + literalLen, packet + index + 1, 4);
            index += 5;
            literalLen += 4;
        }
        literal = binaryToDecimal(literalBits, literalLen);
        printf("Literal: %ld\n", literal);
    } else {
        // operator
        subPacket *subPackets = malloc(sizeof(subPacket) * 100);
        int64_t subPacketCount = 0;
        if (packet[index++]) {
            int64_t subNum = binaryToDecimal(packet + index, 11);
            index += 11;
            printf("Subpacket count: %ld\n", subNum);
            while (subNum > 0) {
                subPacket subPacketLen = parsePacket(packet + index);
                subPackets[subPacketCount++] = subPacketLen;
                index += subPacketLen.length;
                subNum--;
            }
        } else {
            int64_t subLen = binaryToDecimal(packet + index, 15);
            index += 15;
            printf("Subpacket length: %ld\n", subLen);
            while (subLen > 0) {
                subPacket subPacketLen = parsePacket(packet + index);
                subPackets[subPacketCount++] = subPacketLen;
                index += subPacketLen.length;
                subLen -= subPacketLen.length;
            }
        }
        result = 0;
        switch (packetType) {
        case 0: // sum
            result = sum(subPackets, subPacketCount);
            break;
        case 1: // product
            result = product(subPackets, subPacketCount);
            break;
        case 2: // min
            result = min(subPackets, subPacketCount);
            break;
        case 3: // max
            result = max(subPackets, subPacketCount);
            break;
        case 5: // greater than
            result = greaterThan(subPackets, subPacketCount) ? 1 : 0;
            break;
        case 6: // less than
            result = lessThan(subPackets, subPacketCount) ? 1 : 0;
            break;
        case 7: // equal
            result = equalTo(subPackets, subPacketCount) ? 1 : 0;
            break;
        }
    }
    printf("END PACKET\n");
    return (subPacket){isLiteral ? literal : result, index};
}

int main() {
    FILE *fp = fopen(FILE_NAME, "r");

    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, fp);

    bool lineBits[strlen(line) * 4];
    int64_t bitsLen = 0;
    int64_t index = 0;
    while (line[index] != '\n') {
        char c = line[index];
        bool *binary = decimalToBinary(strtol(&c, NULL, 16));
        memcpy(lineBits + index * 4, binary, 4);
        bitsLen += 4;
        index++;
    }

    for (int64_t i = 0; i < bitsLen; i++) {
        printf("%d", lineBits[i]);
    }
    printf("\n");

    subPacket packetLen = parsePacket(lineBits);
    printf("Packet length: %ld\n", packetLen.length);
    printf("Packet value: %ld\n", packetLen.value);
}
