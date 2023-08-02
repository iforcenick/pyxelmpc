#import "pyxel-event-emitter.h"

static PyxelEventEmitter* eventEmitterInstance = nil;

@implementation PyxelEventEmitter

RCT_EXPORT_MODULE();


+ (PyxelEventEmitter*)sharedInstance {
 return eventEmitterInstance;
}
+ (void)setSharedInstance:(PyxelEventEmitter*)instance {
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
