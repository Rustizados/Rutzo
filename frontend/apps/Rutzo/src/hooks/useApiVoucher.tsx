import { useState, useEffect, useMemo, useCallback } from 'react';
import { useVoucher, useBalanceFormat, useApi, useAccount } from '@gear-js/react-hooks';
import { MAIN_CONTRACT, VOUCHER_MIN_LIMIT } from '@/app/consts';
import {
    GearKeyring,
    ProgramMetadata,
} from "@gear-js/api";

import { sleepReact } from '@/app/utils';
 
export function useApiVoucher() {
  const { isVoucherExists, voucherBalance } = useVoucher(MAIN_CONTRACT.PROGRAM_ID);
  const { accounts, account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const [voucher, setVoucher] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [updateVoucher, setUpdateVoucher] = useState(false);
  const { api } = useApi();
  // Datos de cuenta del administrador donde se efectuaran los pagos en los contratos
  // de los nfts y del main contract
  const mnemonic =
    "strong orchard plastic arena pyramid lobster lonely rich stomach label clog rubber";
  const { seed } = GearKeyring.generateSeed(mnemonic);


  const createVoucer2 = new Promise(async (resolve) => {
    if (!account || !api || isVoucherExists == null) {
        console.log("account, api or isVoucherExists not initialized");
        resolve(true);
        return;
    }

    if (isVoucherExists) {
        console.log("Voucher already exists");
        resolve(true);
        return;
    }

    const mainContractVoucher = api.voucher.issue(
        account?.decodedAddress ?? "0x00",
        MAIN_CONTRACT.PROGRAM_ID,
        updateVoucher ? 2_000_000_000_000 : 13_000_000_000_000
        //18000000000000
      );

    const keyring = await GearKeyring.fromSeed(seed, "AdminDavid");

    // Se firma el voucher con la cuenta del administrador para el main Contract

    try {
        await mainContractVoucher.extrinsic.signAndSend(
            keyring,
            async (event) => {
                console.log(event.toHuman());
            }
        );
    } catch (error: any) {
        console.log(`${error.name}: ${error.message}`);
        resolve(true);
        return;
    }

    let voucherExists = false;
    /* eslint-disable no-await-in-loop */
    while (!voucherExists) {
      voucherExists = await api.voucher.exists(MAIN_CONTRACT.PROGRAM_ID, account?.decodedAddress ?? "0x00");
    }

    resolve(false);

  });


  useEffect(() => {
    if (account && isVoucherExists !== undefined) {
      const fetchData = async () => {
        try {
          setIsLoading(true);
          const availableBack = await fetch(ADDRESS.BACK);

          if (availableBack?.status === 200) {
            if (isVoucherExists) {
              setVoucher(true);
            } else {
              const createdVoucher = await createVoucher();
              if (createdVoucher) {
                setVoucher(true);
              }
            }
          }
          setIsLoading(false);
        } catch (error) {
          setIsLoading(false);
        }
      };

      fetchData();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account, isVoucherExists]);

  const updateBalance = useCallback(async () => {
    const formattedBalance = voucherBalance && getFormattedBalanceValue(voucherBalance.toString()).toFixed();
    const isBalanceLow = formattedBalance < VOUCHER_MIN_LIMIT;

    if (isBalanceLow) {
      const createdVoucher = await createVoucher();
      if (createdVoucher) {
        setVoucher(true);
      }
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [voucherBalance]);

  const isVoucher = useMemo(() => voucher, [voucher]);

  return { isVoucher, isLoading, updateBalance };
}