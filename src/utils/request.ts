import { get } from 'lodash-es';
import qs from 'qs';
import type { InternalAxiosRequestConfig } from 'axios';
import axios from 'axios';
import axiosTauriApiAdapter from 'axios-tauri-api-adapter';

const request = axios.create({
  baseURL: 'https://api.unsplash.com',
  timeout: 30000,
  headers: { 'Content-Type': 'application/json' },
  paramsSerializer: {
    serialize(params: object) {
      return qs.stringify(params, { arrayFormat: 'brackets' });
    },
  },
  adapter: axiosTauriApiAdapter,
});

request.interceptors.request.use((config) => {
  if (config.headers && !config.headers.Authorization) {
    config.headers.Authorization = `Client-ID ${import.meta.env.VITE_UNSPLASH_ACCESS_KEY}`;
  }
  return config;
});

request.interceptors.response.use(
  (res: any) => res,
  async (err: { message: any; code: number; status: any;[key: string]: any; config: InternalAxiosRequestConfig }) => {
    const response
      = get(err, 'response', {});
    if (response.data) {
      const { error_message, messages, error, code } = response.data;
      err.message = error_message || messages || error || err.message;
      err.code = code;
    }
    err.status = response.status;
    return Promise.reject(err);
  },
);

export { request };
