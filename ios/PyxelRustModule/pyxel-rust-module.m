#import <React/RCTBridgeModule.h>
#import "pyxel-rust-module.h"
#import "pyxel-event-emitter.h"

typedef void (^StartTimerCallback)(const char *, int);

StartTimerCallback globalStartTimerCallback = NULL;
PyxelEventEmitter *eventEmitter = NULL;

// C function wrapper to call the stored block indirectly
void startTimerCallbackWrapper(const char *message, int duration) {
  if (globalStartTimerCallback) {
    globalStartTimerCallback(message, duration);
  }
}

@interface PyxelRustModule : NSObject <RCTBridgeModule>
@end

@implementation PyxelRustModule

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(startTimer) {
  eventEmitter = [PyxelEventEmitter sharedInstance];
  
  globalStartTimerCallback = ^(const char *message, int duration) {
    [eventEmitter sendEventWithName:@"message" body:@"Event data"];
  };
  
  // Call the original C function
  start_timer(&startTimerCallbackWrapper);
}

@end
