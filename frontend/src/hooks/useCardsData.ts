import {useCallback, useState} from 'react';
import {MAIN_CONTRACT, NFT_CONTRACT} from "@/app/consts";
import {ProgramMetadata} from "@gear-js/api";
import {useAccount, useApi} from "@gear-js/react-hooks";


const useCardsData = () => {
	const {api} = useApi();
	const {account} = useAccount();

	const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
	const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);


	const [allUserCards, setAllUserCards] = useState([]);
	const [playingUserCards, setPlayingUserCards] = useState("");


	const getMatchId = async () => {
		if (!api || !account) return;

		const payload = account?.decodedAddress ? {PlayerIsInMatch: account.decodedAddress} : null;

		if (!payload) return;

		const response = await api.programState.read({
			programId: MAIN_CONTRACT.PROGRAM_ID,
			payload
		}, mainContractMetadata);

		const {playerInMatch}: any = response.toJSON();

		return playerInMatch ?? -1;

	}


	const fetchData = useCallback(async () => {
		if (!account || !api) return;

		try {
			const response = await api.programState.read({
				programId: NFT_CONTRACT.PROGRAM_ID,
				payload: {tokensForOwner: account?.decodedAddress ?? "0x0"}
			}, nftContractMetadata);

			const formatedResponse: any = await response.toJSON();

			setAllUserCards(formatedResponse.tokensForOwner ?? []);
		} catch (error) {
			console.error(error);
		}
	}, []);

	const getPlayingCards = useCallback(async () => {
		if (!account || !api) return;
		const matchId = await getMatchId();

		try {
			const response = await api.programState.read({
				programId: MAIN_CONTRACT.PROGRAM_ID,
				payload: {GameInformationById: [matchId]}
			}, mainContractMetadata);

			const formatedResponse: any = await response.toJSON();
			console.log("formatedResponse", formatedResponse);

			const {chosenNft: tokenId} = formatedResponse.gameInformation.user1;
			/* TODO: Se puede tomar la info directamente */

			setPlayingUserCards(tokenId);


		} catch (error) {
			console.error(error);
		}

	}, []);

	return {allUserCards, fetchData, getPlayingCards, playingUserCards};
}


export default useCardsData;
