export interface Config {
  interval: number;
  key: string;
}

export function getConfig(): Config {
  return {
    interval: 1800,
    key: '',
  };
}
