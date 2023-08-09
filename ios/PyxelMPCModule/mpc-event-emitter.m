#import "mpc-event-emitter.h"

static MPCEventEmitter* eventEmitterInstance = nil;

@implementation MPCEventEmitter

RCT_EXPORT_MODULE();


+ (MPCEventEmitter*)sharedInstance {
 return eventEmitterInstance;
}
+ (void)setSharedInstance:(MPCEventEmitter*)instance {
  eventEmitterInstance = instance;
}

- (instancetype)init {
  self = [super init];
  eventEmitterInstance = self;
  return self;
}

- (NSArray<NSString *> *)supportedEvents {
  // Define the list of events that can be emitted
  return @[@"message"];
}

// - (void)sendEventWithName:(NSString *)name body:(id)body {
//   // Override the default `sendEventWithName:body:` method to emit events
//   [super sendEventWithName:name body:body];
// }

@end
