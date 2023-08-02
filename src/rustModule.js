import {NativeModules, NativeEventEmitter} from 'react-native';

const {PyxelRustModule, PyxelEventEmitter} = NativeModules;
const pyxelEventEmitter = new NativeEventEmitter(PyxelEventEmitter);
console.log(pyxelEventEmitter);
const startTimer = callback => {
  pyxelEventEmitter.addListener('message', msg => callback(msg));
  PyxelRustModule.startTimer();
};

export default startTimer;
