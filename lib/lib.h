#pragma once

#include <stdint.h>

typedef struct
{
    uint16_t length;
    int data[0];
} Packet;

uint16_t get_packet_len(void *packet_ptr);
