import { MAIN_CONTRACT } from '@/app/consts';
import { useApi, useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { DefaultNfts, RegisterButton, NftsOnSale } from '@/components';
import { ProgramMetadata } from '@gear-js/api';
import './Marketplace.scss';

function Marketplace() {
  const { api } = useApi();
  const { account } = useAccount();
  const [isRegister, setIsRegister] = useState(false);
  const [totalNftsToMint, setTotalNftsToMint] = useState(5);

  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);

  const setData = async () => {
    if (!api) return;

    const stateResult = await api.programState.read(
      { programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? '0x0' } },
      mainContractMetadata,
    );

    const stateFormated: any = stateResult.toJSON();

    setIsRegister(stateFormated.userIsRegister);

    if (!isRegister) return;

    const stateResult2 = await api.programState.read(
      { programId: MAIN_CONTRACT.PROGRAM_ID, payload: { NFTsPurchasedByUser: account?.decodedAddress ?? '0x0' } },
      mainContractMetadata,
    );

    const stateFormated2: any = stateResult2.toJSON();

    const mintedNfts: [number] = stateFormated2.purchasedNfts;

    setTotalNftsToMint(3 - (mintedNfts?.length ?? 0));
  };

  setData();

  return (
    <div className="text-center">
      <h1 className=" text-3xl md:text-5xl font-semibold mb-6 ">
        Explore <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">Marketplace</span>
      </h1>
      <p className="text-xl">Get ready for the battle with some cool NFTs</p>
      {isRegister ? (
        <>
          {totalNftsToMint > 0 && (
            <h2 className="m-5 text-xl">
              <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-lg p-1">Free NFTs!</span> pick{' '}
              {totalNftsToMint}
            </h2>
          )}
          <div>
            <div className="cards-container">
              <DefaultNfts onMinted={setData} />
              <br />
              {totalNftsToMint === 0 && <NftsOnSale />}
            </div>
          </div>
        </>
      ) : (
        <>
          <h2 className="m-5 text-xl">To get nfts you must register!</h2>
          <div className="empty_container">
            <RegisterButton onRegister={setData} />
          </div>
        </>
      )}
    </div>
  );
}

export { Marketplace };
