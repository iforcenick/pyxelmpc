#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface MPCEventEmitter : RCTEventEmitter <RCTBridgeModule>
+ (MPCEventEmitter*)sharedInstance;
+ (void)setSharedInstance:(MPCEventEmitter*)instance;
@end
