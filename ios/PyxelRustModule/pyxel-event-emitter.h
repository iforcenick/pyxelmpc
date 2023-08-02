#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface PyxelEventEmitter : RCTEventEmitter <RCTBridgeModule>
+ (PyxelEventEmitter*)sharedInstance;
+ (void)setSharedInstance:(PyxelEventEmitter*)instance;
@end
