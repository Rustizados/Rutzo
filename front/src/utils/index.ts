import { GasInfo } from '@gear-js/api';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'consts';

const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

const gasToSpend = (gasInfo: GasInfo): bigint => {
    const gasHuman = gasInfo.toHuman();
    const minLimit = gasHuman.min_limit?.toString() ?? "0";
    const gasLimit: bigint = BigInt(minLimit.replaceAll(',', ''));
    return gasLimit;
}

export { isLoggedIn, gasToSpend };
