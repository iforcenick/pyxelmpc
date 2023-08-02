import React, {useEffect, useState} from 'react';
import {Text, View} from 'react-native';
import startTimer from './src/rustModule';

const App = () => {
  const [data, setData] = useState('');

  useEffect(() => {
    const callback = (msg) => {
      console.log(msg);
      // setData(prevData => prevData + receivedMessage);
    };

    startTimer(callback);

    return () => {
      // Clean up logic if necessary
    };
  }, []);

  return (
    <View>
      <Text>ABC</Text>
      <Text>{data}</Text>
    </View>
  );
};

export default App;
