import { useState } from 'react';
import { useAlert, useApi, useAccount } from '@gear-js/react-hooks';
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { ProgramMetadata } from "@gear-js/api";
import { sleepReact } from "@/app/utils";

export const useGameLogic = () => {

    const alert = useAlert();

    const { api } = useApi();
    const { account } = useAccount();
    const [userPressPlayButton, setUserPressPlayButton] = useState(false);  // -----
    const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]); // ----
    const [selectedCards, setSelectedCards] = useState<any>([]); // ----
    const [cardToPlay, setCardToPlay] = useState<any | null>(null); // -----
    const [nftsLoaded, setNftsLoaded] = useState(false);  // -----
    const [userInMatch, setUserInMatch] = useState(false);  // ------
    const [matchInProgress, setMatchInProgress] = useState(false);  // ------
    const [actualUserInMatch, setActualUserInMatch] = useState("0x00");  // ----
    const [enemyCard, setEnemyCard] = useState<any | null>(null);  // ----
    const [userWonTheMatch, setUserWonTheMatch] = useState<boolean | null>(false);  //-- --

    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
    const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

    const resetBoard = () => {
        setTokensForOwnerState([]);
        setSelectedCards([]);
        setCardToPlay(null);
        setUserInMatch(false);
        setMatchInProgress(false);
        setNftsLoaded(false);
        setUserPressPlayButton(false);
        setActualUserInMatch(account?.decodedAddress ?? "0x00");
        setUserWonTheMatch(false);
        setEnemyCard(null);
    }

    const ActualMatchOfUser = async (): Promise<number> => {
        if (!api) return -1;
        const stateResult = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { PlayerIsInMatch: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);

        const stateFormated: any = stateResult.toJSON();

        return stateFormated.playerInMatch ?? -1;
    }

    const lastMatchOfUser = async (): Promise<number> => {
        if (!api) return -1;
        const stateResult = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { PlayerInformation: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);

        const stateFormated: any = stateResult.toJSON();

        return stateFormated.playerInformation.recentPastGame ?? -1;
    }

    const setActualSelectedCardFromMatch = async (matchId: number) => {
        if (!api) return;
        const stateResult = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { GameInformationById: [matchId] } }, mainContractMetadata);

        const stateFormated: any = stateResult.toJSON();

        const { user1 } = stateFormated.gameInformation;
        const tokenId = user1.chosenNft;

        if (tokensForOwnerState.length === 0) return;

        const selectedNft = tokensForOwnerState.find((nft: any) => nft[0] === tokenId);

        setCardToPlay(selectedNft);

        setTokensForOwnerState(
            tokensForOwnerState.filter((nft: any) => nft[0] !== tokenId)
        );

        setMatchInProgress(true);
    }



    const showMatchResults = (userAddress: `0x${string}`, matchData: any) => {
        const matchStateData = matchData.matchState;
        const user1Data = matchData.user1;
        const user2Data = matchData.user2;
        const cardToShow = user1Data.userId === userAddress
            ? user2Data.nftData
            : user1Data.nftData;

        if (Object.keys(matchStateData)[0] !== "draw") {
            const wonTheMatch = matchStateData.finished.winner === userAddress;
            setUserWonTheMatch(wonTheMatch);
        } else {
            setUserWonTheMatch(null);
        }

        setEnemyCard(cardToShow);
    }

    const userWaitingMatch = async (matchId: number) => {
        if (!api) return;

        let matchFinished = false;

        /* eslint-disable no-await-in-loop */
        while (!matchFinished) {
            const stateResult = await api
                .programState
                .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [matchId] } }, mainContractMetadata);

            const stateFormated: any = stateResult.toJSON();
            const status = Object.keys(stateFormated)[0];
            if (status === 'matchDoesNotExists') {
                break;
            }

            const matchState = Object.keys(stateFormated.matchState)[0];

            if (matchState !== 'inProgress') {
                matchFinished = true;
            }
        }

        const matchInformationStateResult = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { GameInformationById: [matchId] } }, mainContractMetadata);

        const matchInformationState: any = matchInformationStateResult.toJSON();

        showMatchResults(
            account?.decodedAddress ?? "0x00",
            matchInformationState.gameInformation
        );
        setMatchInProgress(false);

        await sleepReact(4000);

        const stateResult = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [matchId] } }, mainContractMetadata);
        const stateFormated: any = stateResult.toJSON();
        const matchState = Object.keys(stateFormated.matchState)[0];

        if (matchState === 'inProgress') {
            alert.error("Erron in contract, searching match");

            await userWaitingMatch(matchId);
            return;
        }

        resetBoard();
    }

    const handlePlayButton = async () => {
        if (!api) return;

        const matchId = await ActualMatchOfUser();

        setUserPressPlayButton(true);

        if (matchId !== -1) {
            setUserInMatch(true);
            setMatchInProgress(true);

            await userWaitingMatch(matchId);
            return;
        }

        setUserInMatch(true);

        const lastMatchId = await lastMatchOfUser();

        const matchInformationStateResponse = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [lastMatchId] } }, mainContractMetadata);

        const matchInformationState: any = matchInformationStateResponse.toJSON();
        const matchState = Object.keys(matchInformationState.matchState)[0];

        if (matchState === 'inProgress') {
            alert.error("Erron in contract!, joined to match");
            resetBoard();
            return;
        }

        const stateResult = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { GameInformationById: [lastMatchId] } }, mainContractMetadata);
        const stateFormated: any = stateResult.toJSON();


        showMatchResults(
            account?.decodedAddress ?? "0x00",
            stateFormated.gameInformation
        );
        setMatchInProgress(false);

        await sleepReact(4000);

        resetBoard();
    }


    const setStateWithoutSelectedCards = (cards: [any], cardsSelected: [any]) => {
        const cardsLeft = cards.filter((card) => {
            const cardSelected = cardsSelected.find((selectedCard) => selectedCard[0] === card[0]);
            return cardSelected === undefined;
        });
        setTokensForOwnerState(cardsLeft);
    }

    const addCardToPlay = (card: any) => {
        if (userInMatch) return;

        const cardsSelected = selectedCards.filter((actualCard: any) => actualCard[0] !== card[0]);

        if (cardToPlay) cardsSelected.push(cardToPlay);
        setCardToPlay(card);
        setSelectedCards(cardsSelected);
    }

    const removeCardToPlay = (card: any) => {
        if (userInMatch || matchInProgress) return;
        setSelectedCards([card, ...selectedCards]);
        setCardToPlay(null);
    }

    const cardSelected = (tokenId: any, selected: boolean) => {
        if (!selected) {
            const nftSelected = tokensForOwnerState.find((token: any) => token[0] === tokenId);
            const actualSelectedCards = [nftSelected, ...selectedCards];
            let actualTokensCards = tokensForOwnerState.filter((token: any) => token[0] !== tokenId);
            if (actualSelectedCards.length > 3) {
                actualTokensCards = [actualSelectedCards.pop(), ...actualTokensCards];
            }
            setTokensForOwnerState(
                actualTokensCards
            );
            setSelectedCards(
                actualSelectedCards
            );
            return;
        }
        const nftSelected = selectedCards.find((token: any) => token[0] === tokenId);
        setSelectedCards(
            selectedCards.filter((token: any) => token[0] !== tokenId)
        );
        setTokensForOwnerState([nftSelected, ...tokensForOwnerState]);
    }

    const setData = async () => {
        if (!api) return;

        if (actualUserInMatch !== account?.decodedAddress) {
            resetBoard();
        }

        if (!nftsLoaded) {

            const resultNfts = await api.programState
                .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, nftContractMetadata);

            const nftStateFormated: any = resultNfts.toJSON();

            const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

            setStateWithoutSelectedCards(tokensForOwner, selectedCards);
            // setTokensForOwnerState(tokensForOwner);

            setNftsLoaded(true);
        }

        if (!userInMatch) {
            const matchId = await ActualMatchOfUser();

            if (matchId !== -1) {
                await setActualSelectedCardFromMatch(matchId);
                setUserInMatch(true);
                setMatchInProgress(true);
            }
            setActualUserInMatch(account?.decodedAddress ?? "0x00");
        }
    };

    setData();


    return {
        userPressPlayButton,
        tokensForOwnerState,
        selectedCards,
        cardToPlay,
        nftsLoaded,
        userInMatch,
        matchInProgress,
        actualUserInMatch,
        enemyCard,
        userWonTheMatch,
        handlePlayButton,
        addCardToPlay,
        removeCardToPlay,
        cardSelected,
        resetBoard
    };
};

export default useGameLogic;
