#pragma once

#include <stdint.h>
#include <stddef.h>

#define MAX_PACKET_SIZE 1024

typedef struct
{
    uint16_t length;
    int data[0];
} Packet;

uint16_t get_packet_len(void *packet_ptr);
