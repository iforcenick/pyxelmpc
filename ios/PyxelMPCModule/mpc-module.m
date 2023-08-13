#import <React/RCTBridgeModule.h>
#import <React/RCTLog.h>

#import "mpc-module.h"
#import "mpc-event-emitter.h"

typedef void (^OutgoingCallback)(const char *, int);

OutgoingCallback globalOutgoingCallback = NULL;
MPCEventEmitter *eventEmitter = NULL;
void *channel = NULL;

// C function wrapper to call the stored block indirectly
void outgoingCallbackWrapper(const char *message, int duration) {
  if (globalOutgoingCallback) {
    globalOutgoingCallback(message, duration);
  }
}

@interface MPCModule : NSObject <RCTBridgeModule>
@end

@implementation MPCModule

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(initChannel) {
  channel = create_channel();
}

RCT_EXPORT_METHOD(dispatchIncoming: (NSString *)message) {
  RCTLogInfo(@"dispatch incoming message start");
  const char *cString = [message UTF8String];
  channel = dispatch_incoming(channel, cString);
  RCTLogInfo(@"dispatch incoming message end");
}

RCT_EXPORT_METHOD(startKeygen: (int)index uniqueIndex: (int)uniqueIndex) {
  eventEmitter = [MPCEventEmitter sharedInstance];
  
  globalOutgoingCallback = ^(const char *message, int messageLen) {
    NSString *msg = [[NSString alloc] initWithCString:message encoding: NSUTF8StringEncoding];
    [eventEmitter sendEventWithName:@"outgoing" body:@{@"data": msg}];
  };
  
  channel = generate_key(channel, index, uniqueIndex, &outgoingCallbackWrapper);
}

RCT_EXPORT_METHOD(createOfflineStage: (int)uniqueIndex localShare: (NSString *)localShare parties: (NSString *)parties) {
  eventEmitter = [MPCEventEmitter sharedInstance];
  
  globalOutgoingCallback = ^(const char *message, int messageLen) {
    NSString *msg = [[NSString alloc] initWithCString:message encoding: NSUTF8StringEncoding];
    [eventEmitter sendEventWithName:@"outgoing" body:@{@"data": msg}];
  };
  
  const char *cLocalShare = [localShare UTF8String];
  const char *cParties = [parties UTF8String];
  channel = create_offline_stage(channel, uniqueIndex, cLocalShare, cParties, &outgoingCallbackWrapper);
}

RCT_EXPORT_METHOD(createSignature: (int)index uniqueIndex: (int)uniqueIndex offlineStage: (NSString *)offlineStage dataToSign: (NSString *)dataToSign) {
  eventEmitter = [MPCEventEmitter sharedInstance];
  
  globalOutgoingCallback = ^(const char *message, int messageLen) {
    NSString *msg = [[NSString alloc] initWithCString:message encoding: NSUTF8StringEncoding];
    [eventEmitter sendEventWithName:@"outgoing" body:@{@"data": msg}];
  };
  
  const char *cOfflineStage = [offlineStage UTF8String];
  const char *cDataToSign = [dataToSign UTF8String];
  channel = create_signature(channel, index, uniqueIndex, cOfflineStage, cDataToSign, &outgoingCallbackWrapper);
}

@end
