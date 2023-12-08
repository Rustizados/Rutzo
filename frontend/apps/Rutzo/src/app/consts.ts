import { HexString } from '@gear-js/api';

interface Contract {
  PROGRAM_ID: `0x${string}`,
  METADATA: string
}

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  BACK: import.meta.env.VITE_BACKEND_ADDRESS,
  GAME: import.meta.env.VITE_CONTRACT_ADDRESS as HexString,
};

export const ROUTES = {
  HOME: '/',
  GAME: '/game',
  NOTFOUND: '*',
};

// Addres are in the mainnet
export const MAIN_CONTRACT: Contract = {
  PROGRAM_ID: import.meta.env.MAIN_CONTRACT_ADDRESS,
  METADATA: import.meta.env.MAIN_CONTRACT_METADATA
};

export const NFT_CONTRACT: Contract = {
  PROGRAM_ID: import.meta.env.NFT_CONTRACT_ADDRESS,
  METADATA: import.meta.env.NFT_CONTRACT_METADATA
};

export const ONE_TVARA_VALUE = 1000000000000;

