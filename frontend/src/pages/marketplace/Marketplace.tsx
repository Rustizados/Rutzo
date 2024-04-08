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
  const [selectedType, setSelectedType] = useState('all');

  const nftTypes = [
    'all',
    'normal',
    'fire',
    'water',
    'electric',
    'grass',
    'ice',
    'fighting',
    'poison',
    'ground',
    'flying',
    'psychic',
    'bug',
    'rock',
    'ghost',
    'dragon',
    'dark',
    'steel',
    'fairy',
  ];

  interface TypeColors {
    [key: string]: string;
  }

  const typeColors: TypeColors = {
    "normal": 'from-gray-400 to-gray-600',
    "fire": 'from-red-400 to-yellow-400',
    "water": 'from-blue-400 to-blue-600',
    "electric": 'from-yellow-400 to-yellow-600',
    "grass": 'from-green-400 to-green-600',
    "ice": 'from-blue-200 to-blue-400',
    "fighting": 'from-red-600 to-red-800',
    "poison": 'from-purple-400 to-purple-600',
    "ground": 'from-yellow-600 to-yellow-800',
    "flying": 'from-blue-300 to-blue-500',
    "psychic": 'from-purple-600 to-purple-800',
    "bug": 'from-green-600 to-green-800',
    "rock": 'from-yellow-800 to-yellow-900',
    "ghost": 'from-purple-700 to-purple-900',
    "dragon": 'from-blue-700 to-blue-900',
    "dark": 'from-gray-700 to-gray-900',
    "steel": 'from-gray-500 to-gray-700',
    "fairy": 'from-pink-400 to-pink-600',
  };

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

  const handleChipClick = (type: string) => {
    setSelectedType(type);
  };

  return (
    <div className="text-center">
      <h1 className=" text-3xl md:text-5xl font-semibold mb-6 ">
        Explore <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl p-1">Marketplace</span>
      </h1>

      <p className="text-xl">Get ready for the battle with some cool NFTs</p>    

      <div className="chips-container flex flex-wrap max-h-32 overflow-y-auto mx-10 justify-center">
        {nftTypes.map((type) => (
          <span
            key={type}
            className={`${(selectedType === type) ? 'bg-gradient-to-r ' + typeColors[type] : ''} m-3 rounded-3xl p-2 cursor-pointer ${(type === "all" && selectedType === "all") ? 'bg-gradient-to-r from-purple-800 to-green-500' : ''}`}
            onClick={() => handleChipClick(type)}>
            {type}
          </span>
        ))}
      </div>
      
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
              {totalNftsToMint === 0 && <NftsOnSale type={selectedType} />}
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
