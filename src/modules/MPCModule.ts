import {NativeModules, NativeEventEmitter} from 'react-native';
import {subscribeRoom, unsubscribeRoom} from '../api';

const {MPCModule, MPCEventEmitter} = NativeModules;
const pyxelEventEmitter = new NativeEventEmitter(MPCEventEmitter);

export type OutgoingMessage = {data_type: string; data: any};
export type OutgoingMessageHandler = (msg: OutgoingMessage) => void;

export const initChannel = async () => {
  await MPCModule.initChannel();
};

export const dispatchIncoming = async (message: string) => {
  await MPCModule.dispatchIncoming(message);
};

export const attachOutgoingListener = (onReceive: OutgoingMessageHandler) => {
  pyxelEventEmitter.addListener('outgoing', ({data}) => {
    const body = JSON.parse(data);
    console.log('receive!!!');
    onReceive(body);
  });
};

export const generateKey = (
  peerIndex: number,
  uniqueIndex: number,
  onBroadcast: (msg: string) => void,
) => {
  return new Promise((resolve, _) => {
    attachOutgoingListener((body: OutgoingMessage) => {
      if (body.data_type === 'broadcast') {
        onBroadcast(body.data as string);
      } else if (body.data_type === 'key') {
        pyxelEventEmitter.removeAllListeners('outgoing');
        resolve(body.data);
      }
    });
    MPCModule.startKeygen(peerIndex, uniqueIndex);
  });
};

export const createOfflineStage = (
  uniqueIndex: number,
  localShare: string,
  parties: number[],
  onBroadcast: (msg: string) => void,
) => {
  return new Promise((resolve, _) => {
    attachOutgoingListener((body: OutgoingMessage) => {
      if (body.data_type === 'broadcast') {
        onBroadcast(body.data as string);
      } else if (body.data_type === 'stage') {
        pyxelEventEmitter.removeAllListeners('outgoing');
        resolve(body.data);
      }
    });
    MPCModule.createOfflineStage(uniqueIndex, localShare, parties.join(','));
  });
};

export const createSignature = (
  index: number,
  uniqueIndex: number,
  offlineStage: string,
  dataToSign: string,
  onBroadcast: (msg: string) => void,
  onPartialSignature: (msg: string) => void,
  onPublicKey: (msg: string) => void,
) => {
  return new Promise((resolve, _) => {
    attachOutgoingListener((body: OutgoingMessage) => {
      if (body.data_type === 'broadcast') {
        onBroadcast(body.data as string);
      } else if (body.data_type === 'partial_signature') {
        onPartialSignature(body.data as string);
      } else if (body.data_type === 'public_key') {
        onPublicKey(body.data as string);
      } else if (body.data_type === 'signature') {
        pyxelEventEmitter.removeAllListeners('outgoing');
        resolve(body.data);
      }
    });
    MPCModule.createSignature(index, uniqueIndex, offlineStage, dataToSign);
  });
};
