import { ProgramMetadata } from "@gear-js/api";
import { useState, useEffect } from 'react';
import { useAccount, useAlert, useApi } from "@gear-js/react-hooks";
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { sleepReact } from "@/app/utils";
import { useDispatch, useSelector } from "react-redux";
import { addCard } from "@/features/cardsSlice";
import { CardProps } from "@/interfaces/Card";

function useGameState() {
    const alert = useAlert();
    const { api } = useApi();
    const { account } = useAccount();
    const [userPressPlayButton, setUserPressPlayButton] = useState(false);
    const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]);
    const [selectedCards, setSelectedCards] = useState<any>([]);
    const [cardToPlay, setCardToPlay] = useState<any | null>(null);
    const [nftsLoaded, setNftsLoaded] = useState(false);
    const [userInMatch, setUserInMatch] = useState(false);
    const [matchInProgress, setMatchInProgress] = useState(false);
    const [actualUserInMatch, setActualUserInMatch] = useState("0x00");
    const [enemyName, setEnemyName] = useState<string | null>(null);
    const [enemyCard, setEnemyCard] = useState<any | null>(null);
    const [enemyCardCount, setEnemyCardCount] = useState<number>(0);
    const [userWonTheMatch, setUserWonTheMatch] = useState<boolean | null>(false);

    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
    const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

    const dispatch = useDispatch();
    const cards = useSelector((state: any) => state.cards.cards);

    const resetBoard = () => {
        setTokensForOwnerState([]);
        setSelectedCards([]);
        setCardToPlay(null);
        setUserInMatch(false);
        setMatchInProgress(false);
        setNftsLoaded(false);
        setUserPressPlayButton(false);
        setActualUserInMatch(account?.decodedAddress ?? "0x00");
        setUserWonTheMatch(null);
        setEnemyCard(null);
        setEnemyName(null);
        setEnemyCardCount(0);
    };

    const getCurrentUserMatch = async (): Promise<number> => {
        if (!api) {
            console.error("API no disponible");
            return -1;
        }

        try {
            const payload = account?.decodedAddress ? { PlayerIsInMatch: account.decodedAddress } : null;
            if (!payload) {
                console.error("Dirección de cuenta no disponible");
                return -1;
            }

            const stateResult = await api.programState.read({
                programId: MAIN_CONTRACT.PROGRAM_ID,
                payload
            }, mainContractMetadata);
            const { playerInMatch }: any = stateResult.toJSON();

            return playerInMatch ?? -1;
        } catch (error) {
            console.error("Error al leer el estado del programa:", error);
            return -1;
        }
    };

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
                payload: { PlayerInformation: address }
            }, mainContractMetadata);
            const { playerInformation }: any = stateResult.toJSON();

            return playerInformation?.recentPastGame ?? -1;
        } catch (error) {
            console.error("Error al leer el estado del programa:", error);
            return -1;
        }
    };

    const getMatchDetails = async (matchId: number) => {
        console.log("getMatchDetails", matchId);
        if (!api) {
            console.error("API no disponible");
            return;
        }

        try {
            console.log(`Obteniendo detalles del match ID: ${matchId}`);
            const response = await api.programState.read({
                programId: MAIN_CONTRACT.PROGRAM_ID,
                payload: { GameInformationById: [matchId] }
            }, mainContractMetadata);
            const formattedState: any = response.toJSON();

            const { user1, user2 } = formattedState.gameInformation;
            const currentUser = account?.decodedAddress;

            const opponent = currentUser === user1.userId ? user2 : user1;
            console.log("opponent", opponent);
            setEnemyName(opponent.userId);

            return formattedState.gameInformation;
        } catch (error) {
            console.error("Error al obtener los detalles de la partida:", error);
        }
    };

    const updateGameWithSelectedCard = async (matchId: number) => {
        if (!api) {
            console.error("API no disponible");
            return;
        }

        try {
            const response = await api.programState.read({
                programId: MAIN_CONTRACT.PROGRAM_ID,
                payload: { GameInformationById: [matchId] }
            }, mainContractMetadata);
            const formattedState: any = response.toJSON();

            const { chosenNft: tokenId } = formattedState.gameInformation.user1;

            if (tokensForOwnerState.length === 0) {
                console.warn("No hay tokens NFT disponibles para el propietario");
                return;
            }

            const selectedNft = tokensForOwnerState.find((nft: any) => nft[0] === tokenId);
            if (!selectedNft) {
                console.warn("NFT seleccionado no encontrado en el estado del propietario");
                return;
            }

            setCardToPlay(selectedNft);
            setTokensForOwnerState(tokensForOwnerState.filter((nft: any) => nft[0] !== tokenId));
            setMatchInProgress(true);
        } catch (error) {
            console.error("Error al actualizar el juego con la tarjeta seleccionada:", error);
        }
    };

    const showMatchResults = (currentUserAddress: `0x${string}`, matchData: any) => {
        const { matchState, user1, user2 } = matchData;
        const opponentData = user1.userId === currentUserAddress ? user2 : user1;
        const opponentCard = opponentData.nftData;
        const opponentCardCount = opponentData.nftCount;

        const isDraw = Object.keys(matchState)[0] === "draw";
        const isWinner = !isDraw && matchState.finished.winner === currentUserAddress;

        setUserWonTheMatch(isDraw ? null : isWinner);
        setEnemyCard(opponentCard);
        setEnemyCardCount(opponentCardCount);
    };

    const userWaitingMatch = async (matchId: number) => {
        if (!api) return;

        let matchFinished = false;

        while (!matchFinished) {
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
            alert.error("Error en el contrato, buscando partida");
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
            console.log(`Match ID encontrado: ${matchId}`);
            setUserInMatch(true);
            setMatchInProgress(true);
            getMatchDetails(matchId); // Obtener detalles del oponente
            await userWaitingMatch(matchId);
            return;
        }

        setUserInMatch(true);

        const lastMatchId = await getLastUserMatch();
        console.log(`Último Match ID: ${lastMatchId}`);

        const matchInformationStateResponse = await api
            .programState
            .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [lastMatchId] } }, mainContractMetadata);

        const matchInformationState: any = matchInformationStateResponse.toJSON();

        const matchState = Object.keys(matchInformationState.matchState)[0];

        if (matchState === 'inProgress') {
            alert.error("Error en el contrato, unido a la partida");
            resetBoard();
            alert.error("Error al unirse al juego, ¡inténtalo de nuevo!");
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
        pushCard(tokenId)
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

    const pushCard = (tokenId: any) => {
        dispatch(addCard(tokenId));
        return true;
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

            console.log("NFTS CARGADOS", nftStateFormated.tokensForOwner ?? [])

            const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

            setStateWithoutSelectedCards(tokensForOwner, selectedCards);
            setNftsLoaded(true);
        }

        if (!userInMatch) {
            const matchId = await getCurrentUserMatch();
            setActualUserInMatch(account?.decodedAddress ?? "0x00");
            await getMatchDetails(matchId);
            if (matchId !== -1) {
                await getMatchDetails(matchId);
                await updateGameWithSelectedCard(matchId);
                setUserInMatch(true);
                setMatchInProgress(true);
                setUserPressPlayButton(true);
                await userWaitingMatch(matchId);
            }
        }
    };

    useEffect(() => {
        if (account && api) {
            setData();
        }
    }, [account, api]);

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
        enemyCardCount,
        userWonTheMatch,
        handlePlayButton,
        cardSelected,
        addCardToPlay,
        removeCardToPlay,
        resetBoard,
        enemyName,
        setUserWonTheMatch
    };
}

export default useGameState;
