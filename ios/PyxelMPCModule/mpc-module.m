#import <React/RCTBridgeModule.h>
#import "mpc-module.h"
#import "mpc-event-emitter.h"

typedef void (^StartTimerCallback)(const char *, int);

StartTimerCallback globalStartTimerCallback = NULL;
MPCEventEmitter *eventEmitter = NULL;

// C function wrapper to call the stored block indirectly
void startTimerCallbackWrapper(const char *message, int duration) {
  if (globalStartTimerCallback) {
    globalStartTimerCallback(message, duration);
  }
}

@interface MPCModule : NSObject <RCTBridgeModule>
@end

@implementation MPCModule

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(startTimer) {
  eventEmitter = [MPCEventEmitter sharedInstance];
  
  globalStartTimerCallback = ^(const char *message, int duration) {
    // [eventEmitter sendEventWithName:@"message" body:@"Event data"];
    [eventEmitter sendEventWithName:@"message" body:message];
  };
  
  // Call the original C function
  start_timer(&startTimerCallbackWrapper);
}

@end
