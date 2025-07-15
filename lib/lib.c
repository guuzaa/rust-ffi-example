#include "lib.h"

uint16_t get_packet_len(void* packet_ptr) {
    if (packet_ptr == NULL) {
        return 0;
    }
    
    Packet* packet = (Packet*)packet_ptr;
    return packet->length;
}
