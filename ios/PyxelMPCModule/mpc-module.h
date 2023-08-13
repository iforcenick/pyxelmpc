#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void *create_channel(void);

void *dispatch_incoming(void *channel, const char *message);

void *generate_key(void *channel, uint16_t index, uint16_t unique_id, void (*callback)(const char*,
                                                                                       int));

void *create_offline_stage(void *channel,
                           uint16_t unique_id,
                           const char *local_share,
                           const char *parties,
                           void (*callback)(const char*, int));

void *create_signature(void *channel,
                       uint16_t index,
                       uint16_t unique_id,
                       const char *offline_stage,
                       const char *data_to_sign,
                       void (*callback)(const char*, int));
