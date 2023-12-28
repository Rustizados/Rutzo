import { useState } from 'react';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { MAIN_CONTRACT, NFT_CONTRACT} from '../app/consts';
import { ProgramMetadata } from "@gear-js/api";
const useContractData = () => {
    const { api } = useApi();
    const { account } = useAccount();
    const [hasEnoughCards, setHasEnoughCards] = useState(false);
    const [numberOfNfts, setNumberOfNfts] = useState(0);
    const [isRegister, setIsRegister] = useState(false);
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);



    const fetchData = async () => {
        if (!account || !api) return;

        const stateResult = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);

        const stateFormated: any = stateResult.toJSON();

        setIsRegister(stateFormated.userIsRegister);

        if (!isRegister) return;

        try {
            const nftStateResult = await api
                .programState
                .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, ProgramMetadata.from(NFT_CONTRACT.METADATA));

            const nftStateFormated: any = nftStateResult.toJSON();

            const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

            const totalNfts = tokensForOwner.length;

            setNumberOfNfts(totalNfts);

            if (totalNfts > 2) {
                setHasEnoughCards(true);
            } else {
                setHasEnoughCards(false);
            }
        } catch (error) {
            console.log(error);
            setHasEnoughCards(false);
        }
    };

    return { hasEnoughCards, fetchData, numberOfNfts, isRegister };
};

export default useContractData;
