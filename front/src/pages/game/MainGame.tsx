import React, { useState, useEffect } from 'react';
import { decodeAddress, ProgramMetadata } from '@gear-js/api';
import { useApi, useAccount } from '@gear-js/react-hooks';
import './slide-in.css';
import './fire.css';
import './selectedCards.css'
import './MainGame.css';
import { cloneDeep } from 'lodash';
import { MySelectedNFT } from './MySelectedNFT';
import { Card } from '../../components/card/Card';
import {Facedowncard} from '../../components/facedowncard/facedowncard';
import {PlayGame} from "./PlayGame";



function InfoNFT({ name, description, media, reference }: any) {
    return (
        <Card image={media} title={name} type={description} value={reference} price={reference} />
    );
}

function CardComponent({ selectedCard }: any) {
    if (!selectedCard) {
        return <h3>Select your Card</h3>;
    }
    return (
        <MySelectedNFT
            name={selectedCard.name}
            description={selectedCard.description}
            media={selectedCard.media}
            reference={selectedCard.reference}
        />
    );
}

function MainGame() {
    const [selectedCards, setSelectedCards] = useState<any[]>([]);
    const [selectedCard, setSelectedCard] = useState();
    const [allNFTs, setAllNFTs] = useState([]);
    const [myNFTCollection, setMyNFTCollection] = useState([]);
    const { api } = useApi();
    const { account } = useAccount();

    const [cardToPlay, setCardToPlay] = useState<any>()

    // const [fullState, setFullState] = useState<any>([]);

    const programIDNFT = '0x43500cceb8e88128eb38ecad188450bcb852e3311ef7ff734d34b11dd48ee651';
    const meta =
        "0001000100000000000108000000010c00000000000000010d000000411d7800082870726f6772616d5f696f40496e6974436f6e74726163744461746100000801306e66745f636f6e747261637404013c4f7074696f6e3c4163746f7249643e00015c746f6b656e735f6d657461646174615f64656661756c741401485665633c546f6b656e4d657461646174613e00000404184f7074696f6e04045401080108104e6f6e6500000010536f6d6504000800000100000810106773746418636f6d6d6f6e287072696d6974697665731c4163746f724964000004000c01205b75383b2033325d00000c000003200000001000100000050300140000021800181020676561725f6c6962486e6f6e5f66756e6769626c655f746f6b656e14746f6b656e34546f6b656e4d6574616461746100001001106e616d651c0118537472696e6700012c6465736372697074696f6e1c0118537472696e670001146d656469611c0118537472696e670001247265666572656e63651c0118537472696e6700001c000005020020082870726f6772616d5f696f2c5275747a6f416374696f6e00011020506c617947616d65080120746f6b656e5f696424011c546f6b656e4964000114706f7765721c0118537472696e67000000204d696e7443617264040120746f6b656e5f69641001087538000100345365744e465441646472657373040008011c4163746f7249640002002052656769737465720003000024083c7072696d69746976655f74797065731055323536000004002801205b7536343b20345d000028000003040000002c002c000005060030082870726f6772616d5f696f285275747a6f4576656e74000158184d696e746564040024011c546f6b656e4964000000404e4654436f6e747261637453617665640001005c4572726f7243616c6c696e674e4654436f6e7472616374000200445573657249734e6f74417070726f766564040008011c4163746f724964000300445573657249734e6f745468654f776e6572040008011c4163746f72496400040020417070726f766564040008011c4163746f7249640005002c4572726f72427579696e6704001c0118537472696e67000600484e46545769746849644e6f74457869737473040010010875380007004c4163636f756e74416c726561647945786973740400080118557365724964000800404163636f756e744e6f744578697374730400080118557365724964000900544163636f756e74416c7265616479496e4d617463680400080118557365724964000a003c51756572794e6f74416c6c6f77656404001c0118537472696e67000b00385265676973746572537563636573000c002c4c6f67696e537563636573000d002c55736572496e4d61746368040008011c4d617463684964000e004c4572726f72496e4a6f696e696e674d61746368000f00385075726368617365537563636573001000305265706c7953756363657373001100244e6577506c61796572040008011c4163746f724964001200344d6174636846696e6973686564001300304d61746368437265617465640014003c4d61784d696e747352656163686564040008011c4163746f724964001500003400000408383c003800000400003c082870726f6772616d5f696f34436f6e7472616374537461746500002001146f776e657208011c4163746f7249640001306e66745f636f6e74726163740401544f7074696f6e3c4e6674436f6e747261637449643e00011475736572734001545665633c285573657249642c20496e47616d65293e00011467616d65734c01545665633c4d61746368496e666f726d6174696f6e3e00015c746f6b656e735f6d657461646174615f64656661756c746001605665633c2875382c20546f6b656e4d65746164617461293e00016c64656661756c745f746f6b656e735f6d696e7465645f62795f69646801585665633c285573657249642c205665633c75383e293e000140617070726f7665645f6d696e7465727374012c5665633c5573657249643e0001387472616e73616374696f6e5f69642c010c753634000040000002440044000004080848004800000500004c000002500050082870726f6772616d5f696f404d61746368496e666f726d6174696f6e00000c0118757365725f315401205573657244617461000118757365725f325801404f7074696f6e3c55736572446174613e00012c6d617463685f73746174655c01284d617463685374617465000054082870726f6772616d5f696f20557365724461746100000c011c757365725f696408011c4163746f72496400012863686f73656e5f6e667424011c546f6b656e4964000114706f776572100108753800005804184f7074696f6e04045401540108104e6f6e6500000010536f6d6504005400000100005c082870726f6772616d5f696f284d61746368537461746500010c2046696e697368656408011877696e6e657208011c4163746f7249640001146c6f73657208011c4163746f72496400000028496e50726f6772657373000100244e6f74457869737473000200006000000264006400000408101800680000026c006c00000408087000700000021000740000020800";

    const metadata = ProgramMetadata.from(meta);
    const currentAccount = account?.address;

    const selectCard = (card: any) => {
        // Crear una copia del estado actual para no mutar el estado directamente
        const updatedSelectedCards = cloneDeep(selectedCards);

        // Buscar si la carta ya ha sido seleccionada
        const cardIndex = updatedSelectedCards.findIndex(item => item.reference === card.reference);

        // Si la carta ya ha sido seleccionada, se elimina de la lista
        if (cardIndex !== -1) {
            updatedSelectedCards.splice(cardIndex, 1);
        } else {
            // Si no ha sido seleccionada y hay menos de 3 cartas, se añade a la lista
            // eslint-disable-next-line no-lonely-if
            if (updatedSelectedCards.length < 3) {
                updatedSelectedCards.push(card);
            }else{
                alert('You can only select 3 cards')
            }

        }

        setSelectedCards(updatedSelectedCards);
        const updatedMyNFTCollection = myNFTCollection.filter(item => !selectedCards.includes(item));
        setMyNFTCollection(updatedMyNFTCollection);
    };

    const selectCardToPlay = (card: any) => {
        if (cardToPlay){
            alert('You can only select 1 card to play')
        }else{
            setCardToPlay(card)
        }

        console.log('cardToPlay', cardToPlay)

    }

    useEffect(() => {
        async function getMyNFT() {
            try {
                const result = await api.programState.read({programId: programIDNFT, payload: ''}, metadata);
                const fullState: any = result.toJSON();
                // Usando encadenamiento opcional y asignando valores predeterminados como arrays vacíos
                const tokensForOwner = fullState.token?.tokensForOwner ?? [];
                console.log('tokensForOwner', tokensForOwner)
                const tokenMetadataById = fullState.token?.tokenMetadataById ?? [];
                console.log('tokenMetadataById', tokenMetadataById)

                const allDataNFTs = tokenMetadataById.map((item: any) => item[1]);
                console.log('allDataNFTs', allDataNFTs)
                setAllNFTs(allDataNFTs);

                // Tomar el primer elemento del resultado de filter, que es un array de arrays con los tokens del owner
                const myNFTsArray = tokensForOwner.filter((obj: any) => obj[0] === decodeAddress(currentAccount ?? ''));
                console.log('myNFTsArray', myNFTsArray)

                // Verificar si myNFTsArray tiene elementos antes de intentar acceder a ellos
                const myNFTs = myNFTsArray[0][1].length > 0 ? myNFTsArray[0][1] : [];
                console.log('myNFTs', myNFTs)

                const myNFTCollectionH = tokenMetadataById
                    .filter((item: any) => myNFTs.includes(item[0]))
                    .map((item: any) => item[1]);

                setMyNFTCollection(myNFTCollectionH);
            } catch (error: any) {
                console.error('Error fetching NFTs:', error);
            }

        }

        getMyNFT();
    }, [api.programState, currentAccount, metadata]);

    const cardStyles = { border: 'none', background: 'transparent', width: '100%', height: '100%' };
    const containerStyles = { display: 'flex', justifyContent: 'center', alignItems: 'center' };



    // @ts-ignore
    return (
        <div>
            <div className="mainContainer" >
            <div style={{ ...containerStyles, flexDirection: 'column' }}>
                <h3 className='sectionTitle'>My NFT Collection</h3>
                <div className='myCardsArea'>
                    {myNFTCollection.length ? (
                        myNFTCollection.map((elemento:any, index: any) => (
                            <button
                                key={crypto.randomUUID()} // considera usar otro identificador único en lugar de randomUUID para una mejor performance
                                type="button"
                                onClick={() => selectCard(elemento)}
                                style={cardStyles}
                            >
                                <InfoNFT
                                    name={elemento.name}
                                    description={elemento.description}
                                    media={elemento.media}
                                    reference={elemento.reference}
                                />
                            </button>
                        ))
                    ) : (
                        <h3 style={{ fontSize: '1.5rem' }}>No NFTs</h3>
                    )}
                </div>
                <h3 className='sectionTitle'>My NFT Selection</h3>
                <div className="selectedCardsArea">
                    <div className="selectedCards">
                        {selectedCards.map((card, index) => (
                            // eslint-disable-next-line react/no-array-index-key
                            <CardComponent key={index} selectedCard={card} />
                        ))}
                    </div>

                    {selectedCards.length === 3 && <a className="buttonPrimary" href="#gamearea">Go!</a>}
                </div>

                <div id="gamearea" className='playArea'>
                    <div className="grid-row opponentArea">
                        {[1,2,3].map((item: number) => (
                                <Facedowncard key={item}/>
                        ))
                        }
                    </div>
                    <div className="versusArea">
                        <div className="grid-row">
                            <CardComponent selectedCard={cardToPlay}/>
                            <Facedowncard />

                        </div>

                        <div className="buttonArea">
                            {cardToPlay && <PlayGame reference={cardToPlay.reference}/>}

                        </div>


                    </div>
                    <div className="grid-row youArea">
                        {selectedCards.map((card, index) => (
                            <button
                                key={crypto.randomUUID()} // considera usar otro identificador único en lugar de randomUUID para una mejor performance
                                type="button"
                                onClick={() => selectCardToPlay(card)}
                                style={cardStyles}
                            >
                            <CardComponent selectedCard={card} />
                            </button>
                        ))}
                    </div>
                </div>


            </div>
            </div>
        </div>
    );
}

export { MainGame };