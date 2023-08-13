import React, {useState} from 'react';
import {Button, Text, View, TextInput, ScrollView} from 'react-native';
import {
  initChannel,
  dispatchIncoming,
  generateKey,
  createOfflineStage,
  createSignature,
} from './src/modules/MPCModule';
import {
  issueUniqueIndex,
  sendOutgoing,
  subscribeRoom,
  unsubscribeRoom,
} from './src/api';
import styles from './styles';

const keygenRoom = 'default-keygen';
const offlineStageRoom = 'default-signing-offline';
const signingRoom = 'default-signing-online';

const App = () => {
  const [peerIndex, setPeerIndex] = useState('1');

  const [privateKey, setPrivateKey] = useState<any>();
  const [keygenState, setKeygenState] = useState<string | null>(null);

  const [offlineStage, setOfflineStage] = useState<any>();
  const [parties, setParties] = useState<string>('1,2');
  const [sigPeerIndex, setSigPeerIndex] = useState<number>(0);
  const [offlineStageState, setOfflineStageState] = useState<string | null>(
    null,
  );

  const [signature, setSignature] = useState<any>();
  const [dataToSign, setDataToSign] = useState<string>('Hello');
  const [partialSignature, setPartialSignature] = useState<string>('');
  const [publicKey, setPublicKey] = useState<string>('');
  const [signatureState, setSignatureState] = useState<string | null>(null);

  const handlePeerIndexChange = (idx: string) => {
    setPeerIndex(idx);
  };
  const handlePartiesChange = (_parties: string) => {
    setParties(_parties);
  };
  const handleDataToSignChange = (_dataToSign: string) => {
    setDataToSign(_dataToSign);
  };

  const handlePressKeygen = async () => {
    await initChannel();
    const messageHashMap: Record<string, boolean> = {};
    await subscribeRoom(keygenRoom, async (message: string) => {
      if (messageHashMap[message]) return;
      messageHashMap[message] = true;
      await dispatchIncoming(message);
    });
    const uniqueIndex = await issueUniqueIndex(keygenRoom);
    setKeygenState('Generating...');
    const key = await generateKey(
      Number(peerIndex),
      Number(uniqueIndex),
      (message: string) => {
        console.log('broadcast: ', message.length);
        sendOutgoing(keygenRoom, message);
      },
    );
    unsubscribeRoom(keygenRoom);
    setPrivateKey(key);
    setKeygenState('Completed');
  };

  const handlePressStage = async () => {
    await initChannel();
    const messageHashMap: Record<string, boolean> = {};
    await subscribeRoom(offlineStageRoom, async (message: string) => {
      if (messageHashMap[message]) return;
      messageHashMap[message] = true;
      await dispatchIncoming(message);
    });
    const uniqueIndex = await issueUniqueIndex(offlineStageRoom);
    setOfflineStageState('Generating...');
    const stage = await createOfflineStage(
      Number(uniqueIndex),
      privateKey,
      parties.split(',').map(p => Number(p.trim())),
      (message: string) => {
        console.log('broadcast: ', message.length);
        sendOutgoing(offlineStageRoom, message);
      },
    );
    unsubscribeRoom(offlineStageRoom);
    setOfflineStage(stage);
    setSigPeerIndex(uniqueIndex);
    setOfflineStageState('Completed');
  };

  const handlePressSign = async () => {
    await initChannel();
    const messageHashMap: Record<string, boolean> = {};
    await subscribeRoom(signingRoom, async (message: string) => {
      if (messageHashMap[message]) return;
      messageHashMap[message] = true;
      const body = JSON.parse(message);
      console.log(
        'received',
        body.sender,
        body.receiver,
        body.body.Round1
          ? 'Round1'
          : body.body.Round2
          ? 'Round2'
          : body.body.Round3
          ? 'Round3'
          : 'Round4',
      );
      await dispatchIncoming(message);
    });
    const uniqueIndex = await issueUniqueIndex(signingRoom);
    setSignatureState('Generating...');
    const _signature = await createSignature(
      sigPeerIndex,
      Number(uniqueIndex),
      offlineStage,
      dataToSign,
      (message: string) => {
        console.log('broadcast: ', message.length);
        sendOutgoing(signingRoom, message);
      },
      (message: string) => {
        setPartialSignature(message);
      },
      (message: string) => {
        setPublicKey(message);
      },
    );
    unsubscribeRoom(signingRoom);
    setSignature(_signature);
    setSignatureState('Completed');
  };

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.header}>Pyxelchain MPC wallet</Text>
      <Text style={styles.subheader}>MVP</Text>

      <View style={styles.panel}>
        <View style={styles.formContainer}>
          <Text style={styles.label}>Peer ID:&nbsp;&nbsp;</Text>
          <TextInput
            style={styles.input}
            value={peerIndex}
            editable={true}
            onChangeText={handlePeerIndexChange}
          />
        </View>
        <View style={styles.seperator} />

        <View style={styles.formContainer}>
          <Text style={styles.label}>Private Key:&nbsp;&nbsp;</Text>
          <TextInput style={styles.input} value={privateKey} editable={false} />
        </View>
        <Button
          title={keygenState || 'Generate key'}
          onPress={handlePressKeygen}
          disabled={keygenState !== null}
        />
        <View style={styles.seperator} />

        <View style={styles.formContainer}>
          <Text style={styles.label}>Parties:&nbsp;&nbsp;</Text>
          <TextInput
            style={styles.input}
            value={parties}
            onChangeText={handlePartiesChange}
          />
        </View>
        <View style={styles.formContainer}>
          <Text style={styles.label}>Offline Stage:&nbsp;&nbsp;</Text>
          <TextInput
            style={styles.input}
            value={offlineStage}
            editable={false}
          />
        </View>
        <Button
          title={offlineStageState || 'Get Offline Stage'}
          onPress={handlePressStage}
          disabled={offlineStageState !== null}
        />
        <View style={styles.seperator} />

        <View style={styles.formContainer}>
          <Text style={styles.label}>Message:&nbsp;&nbsp;</Text>
          <TextInput
            style={styles.input}
            value={dataToSign}
            onChangeText={handleDataToSignChange}
          />
        </View>
        <View style={styles.formContainer}>
          <Text style={styles.label}>Partial Signature:&nbsp;&nbsp;</Text>
          <TextInput
            style={styles.input}
            value={partialSignature}
            editable={false}
          />
        </View>
        <View style={styles.formContainer}>
          <Text style={styles.label}>Public Key:&nbsp;&nbsp;</Text>
          <TextInput style={styles.input} value={publicKey} editable={false} />
        </View>
        <View style={styles.formContainer}>
          <Text style={styles.label}>Signature:&nbsp;&nbsp;</Text>
          <TextInput style={styles.input} value={signature} editable={false} />
        </View>
        <Button
          title={signatureState || 'Sign Message'}
          onPress={handlePressSign}
          disabled={signatureState !== null}
        />
      </View>
    </ScrollView>
  );
};

export default App;
