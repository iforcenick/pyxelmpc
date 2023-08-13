import {StyleSheet} from 'react-native';

const styles = StyleSheet.create({
  container: {
    flex: 1,
    overflow: 'hidden',
    marginTop: 80,
    width: '100%',
  },
  header: {
    fontSize: 20,
    textAlign: 'center',
  },
  subheader: {
    marginTop: 10,
    fontSize: 15,
    textAlign: 'center',
  },

  panel: {
    marginTop: 20,
    padding: 20,
  },
  peerIndex: {
    fontSize: 15,
  },

  seperator: {
    width: '100%',
    paddingHorizontal: 20,
    height: 1,
    backgroundColor: '#00000088',
    marginVertical: 30,
  },

  formContainer: {
    width: '100%',
    flexDirection: 'row',
    marginTop: 10,
  },
  label: {
    paddingTop: 10,
  },
  input: {
    flex: 1,
    backgroundColor: '#00000011',
    padding: 10,
  },
});

export default styles;
