import { ProgramMetadata } from "@gear-js/api";
import { useState } from 'react';
import { useApi, useAccount, useAlert } from "@gear-js/react-hooks";
import {MAIN_CONTRACT, NFT_CONTRACT} from "@/app/consts";
import {sleepReact} from "@/app/utils";

function useGameState() {
    const alert = useAlert();
    const {api} = useApi();
    const {account} = useAccount();
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
    // ... all your state and functions here

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

    // Get the current user's match
    const getCurrentUserMatch = async (): Promise<number> => {
        if (!api) {
            console.error("API no disponible");
            return -1;
        }

        try {
            const payload = account?.decodedAddress ? {PlayerIsInMatch: account.decodedAddress} : null;
            if (!payload) {
                console.error("Dirección de cuenta no disponible");
                return -1;
            }

            const stateResult = await api.programState.read({
                programId: MAIN_CONTRACT.PROGRAM_ID,
                payload
            }, mainContractMetadata);
            const {playerInMatch}: any = stateResult.toJSON();

            return playerInMatch ?? -1;
        } catch (error) {
            console.error("Error al leer el estado del programa:", error);
            return -1;
        }
    };

    // Get the last user's match
    const getLastUserMatch = async (): Promise<number> => {
        if (!api) {
            console.error("API no disponible");
            return -1;
        }

        try {
            const address = account?.decodedAddress ?? "0x0";
            if (address === "0x0") {
                console.error("Dirección de cuenta no disponible");
                return -1;
            }

            const stateResult = await api.programState.read({
                programId: MAIN_CONTRACT.PROGRAM_ID,
                payload: {PlayerInformation: address}
            }, mainContractMetadata);
            const {playerInformation}: any = stateResult.toJSON();

            return playerInformation?.recentPastGame ?? -1;
        } catch (error) {
            console.error("Error al leer el estado del programa:", error);
            return -1;
        }
    };


    // Update the game with the selected card by the user
    const updateGameWithSelectedCard = async (matchId: number) => {
        if (!api) {
            console.error("API no disponible");
            return;
        }

        try {
            const response = await api.programState.read({
                programId: MAIN_CONTRACT.PROGRAM_ID,
                payload: {GameInformationById: [matchId]}
            }, mainContractMetadata);
            const formattedState: any = response.toJSON();

            const {chosenNft: tokenId} = formattedState.gameInformation.user1;

            if (tokensForOwnerState.length === 0) {
                console.warn("No hay tokens NFT disponibles para el propietario");
                return;
            }

            const selectedNft = tokensForOwnerState.find((nft: any) => nft[0] === tokenId);
            if (!selectedNft) {
                console.warn("NFT seleccionado no encontrado en el estado del propietario");
                return;
            }

            // Actualizar el juego con la tarjeta seleccionada
            setCardToPlay(selectedNft);
            // Eliminar la tarjeta seleccionada del estado del propietario
            setTokensForOwnerState(tokensForOwnerState.filter((nft: any) => nft[0] !== tokenId));
            // Actualizar el estado del juego
            setMatchInProgress(true);
        } catch (error) {
            console.error("Error al actualizar el juego con la tarjeta seleccionada:", error);
        }
    };

    // Show the match results
    const showMatchResults = (currentUserAddress : `0x${string}`, matchData: any) => {
        const { matchState, user1, user2 } = matchData;
        const opponentData = user1.userId === currentUserAddress ? user2 : user1;
        const opponentCard = opponentData.nftData;

        const isDraw = Object.keys(matchState)[0] === "draw";
        const isWinner = !isDraw && matchState.finished.winner === currentUserAddress;

        setUserWonTheMatch(isDraw ? null : isWinner);
        setEnemyCard(opponentCard);
    }


    const userWaitingMatch = async (matchId: number) => {
        if (!api) return;

        let matchFinished = false;

        /* eslint-disable no-await-in-loop */
        while (!matchFinished) {
            // console.log("Buscando partida!!");

            const stateResult = await api
                .programState
                .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [matchId] } }, mainContractMetadata);

            const stateFormated: any = stateResult.toJSON();
            const status = Object.keys(stateFormated)[0];
            if (status === 'matchDoesNotExists') {
                console.log("La partida no existe!!");
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

        const matchId = await getCurrentUserMatch();
        setUserPressPlayButton(true);

        if (matchId !== -1) {
            setUserInMatch(true);
            setMatchInProgress(true);
            await userWaitingMatch(matchId);
            return;
        }

        setUserInMatch(true);

        const lastMatchId = await getLastUserMatch();

        const matchInformationStateResponse = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [lastMatchId] } }, mainContractMetadata);

        const matchInformationState: any = matchInformationStateResponse.toJSON();

        const matchState = Object.keys(matchInformationState.matchState)[0];

        if (matchState === 'inProgress') {
            alert.error("Erron in contract!, joined to match");
            resetBoard();
            alert.error("Error joining the game, try again!");
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
            console.log("Se va a formatear el tablero este!");

            resetBoard();
        }

        if (!nftsLoaded) {
            console.log("CARGANDO NFTS");

            const resultNfts = await api.programState
                .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, nftContractMetadata);

            const nftStateFormated: any = resultNfts.toJSON();

            const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

            setStateWithoutSelectedCards(tokensForOwner, selectedCards);
            // setTokensForOwnerState(tokensForOwner);

            setNftsLoaded(true);
        }

        if (!userInMatch) {
            const matchId = await getCurrentUserMatch();
            setActualUserInMatch(account?.decodedAddress ?? "0x00");
            if (matchId !== -1) {
                await updateGameWithSelectedCard(matchId);
                setUserInMatch(true);
                setMatchInProgress(true);
                setUserPressPlayButton(true);
                await userWaitingMatch(matchId);
            }
        }
    };

    // Load initial data and set up any listeners
    setData();


    // Export state and actions
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
        cardSelected,
        addCardToPlay,
        removeCardToPlay,
        resetBoard
    };
}

export default useGameState;
