import EventSource, {EventType} from 'react-native-sse';
import {BASE_URL} from './constants';

const roomMap: any = {};

export const subscribeRoom = (roomId: string, onMessage: any) => {
  return new Promise((resolve, _) => {
    const source = new EventSource(
      // `http://127.0.0.1:8000/rooms/${roomId}/subscribe`,
      `${BASE_URL}/${roomId.toString()}/subscribe`,
    );

    source.addEventListener('open', () => {
      console.log('Room subscribed.');
      resolve(null);
    });

    source.addEventListener('new-message' as EventType, function (e: any) {
      onMessage(e.data as string);
    });
    roomMap[roomId] = source;
  });
};

export const unsubscribeRoom = (roomId: string) => {
  const source: EventSource = roomMap[roomId];
  source.removeAllEventListeners();
};
