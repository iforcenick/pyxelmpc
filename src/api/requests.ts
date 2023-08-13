import axios from 'axios';
import {BASE_URL} from './constants';

export const myAxios = axios.create({
  baseURL: BASE_URL,
});

export const issueUniqueIndex = async (roomId: string) => {
  const {data} = await myAxios.post(`/${roomId}/issue_unique_idx`);
  return Number(data.unique_idx) as number;
};

export const sendOutgoing = async (roomId: string, message: string) => {
  await myAxios.post(`/${roomId}/broadcast`, message);
};
