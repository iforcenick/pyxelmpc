import {NativeModules, NativeEventEmitter} from 'react-native';

const {MPCModule, MPCEventEmitter} = NativeModules;
const pyxelEventEmitter = new NativeEventEmitter(MPCEventEmitter);
console.log(pyxelEventEmitter);
const startTimer = callback => {
  pyxelEventEmitter.addListener('message', msg => callback(msg));
  MPCModule.startTimer();
};

export default startTimer;
