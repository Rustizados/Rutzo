import { useState } from "react";
import { decodeAddress, ProgramMetadata } from "@gear-js/api";
import { useApi, useAccount } from "@gear-js/react-hooks";
import "./slide-in.css";
import "./fire.css";
import { MySelectedNFT } from "./MySelectedNFT";
import { Card } from '../../components/card/Card';

function InfoNFT({ name, description, media, reference }: any) {
  return (
    <Card image={media} title={name} type={description} value={reference} price={reference}/>
  );
}

function CardComponent({
  card1State,
  card2State,
  card3State,
  myselectednft,
  myselectednft2,
  myselectednft3,
}: any) {
  let cardComponent;

  if (card1State === 1) {
    cardComponent = (
      <MySelectedNFT
        name={myselectednft.name}
        description={myselectednft.description}
        media={myselectednft.media}
        reference={myselectednft.reference}
      />
    );
  } else if (card2State === 2) {
    cardComponent = (
      <MySelectedNFT
        name={myselectednft2.name}
        description={myselectednft2.description}
        media={myselectednft2.media}
        reference={myselectednft2.reference}
      />
    );
  } else if (card3State === 3) {
    cardComponent = (
      <MySelectedNFT
        name={myselectednft3.name}
        description={myselectednft3.description}
        media={myselectednft3.media}
        reference={myselectednft3.reference}
      />
    );
  } else {
    cardComponent = <h3>Select your Card</h3>;
  }

  return cardComponent;
}

function MainGame() {
  const [card1State, setCard1State] = useState(0);

  const [card2State, setCard2State] = useState(0);

  const [card3State, setCard3State] = useState(0);

  const firstinterruptor = () => {
    setCard1State(1);
    setCard2State(0);
    setCard3State(0);
  };

  const secondinterruptor = () => {
    setCard1State(0);
    setCard2State(2);
    setCard3State(0);
  };

  const thirdinterruptor = () => {
    setCard1State(0);
    setCard2State(0);
    setCard3State(3);
  };

  const restart = () => {
    setCard1State(0);
    setCard2State(0);
    setCard3State(0);
  };

  const { api } = useApi();
  const { account } = useAccount();

  const [allnfts, setAllnfts] = useState<any | undefined>([]);
  const [allmynft, setAllmynft] = useState<any | undefined>();
  const [existNFT, setExistNFT] = useState<any | undefined>(true);
  const [myselectednft, setMyselectednft] = useState<any | undefined>([]);
  const [myselectednft2, setMyselectednft2] = useState<any | undefined>([]);
  const [myselectednft3, setMyselectednft3] = useState<any | undefined>([]);
  const [tokensForOwnerState, setTokensForOwnerState] = useState<
    any | undefined
  >([]);
  const [fullState, setFullState] = useState<any | undefined>({});

  const [mynftcollection, setMynftcollection] = useState<any | undefined>([]);

  const alldatanfts: any[] = [];

  const mynftscollection: any[] = [];

  // Add your programID
  const programIDNFT =
    "0x8ecff0491f294b0f885f8234ec0c94d1712670dc5530b8a9804d876102e86331";

  // Add your metadata.txt
  const meta =
    "0001000100000000000111000000011900000000000000011e000000212db00008186e66745f696f1c496e69744e465400000c0128636f6c6c656374696f6e040128436f6c6c656374696f6e000124726f79616c746965730c01444f7074696f6e3c526f79616c746965733e00012c636f6e73747261696e747334012c436f6e73747261696e747300000408186e66745f696f28436f6c6c656374696f6e00000801106e616d65080118537472696e6700012c6465736372697074696f6e080118537472696e6700000800000502000c04184f7074696f6e04045401100108104e6f6e6500000010536f6d650400100000010000101020676561725f6c6962486e6f6e5f66756e6769626c655f746f6b656e24726f79616c7469657324526f79616c7469657300000801206163636f756e74731401185061796f757400011c70657263656e7430010c753136000014042042547265654d617008044b011804560124000400280000001810106773746418636f6d6d6f6e287072696d6974697665731c4163746f724964000004001c01205b75383b2033325d00001c000003200000002000200000050300240000050700280000022c002c000004081824003000000504003408186e66745f696f2c436f6e73747261696e747300000801386d61785f6d696e745f636f756e7438012c4f7074696f6e3c7533323e000148617574686f72697a65645f6d696e746572734001305665633c4163746f7249643e00003804184f7074696f6e040454013c0108104e6f6e6500000010536f6d6504003c00000100003c00000505004000000218004408186e66745f696f244e4654416374696f6e00012c104d696e740801387472616e73616374696f6e5f696448010c753634000138746f6b656e5f6d657461646174614c0134546f6b656e4d65746164617461000000104275726e0801387472616e73616374696f6e5f696448010c753634000120746f6b656e5f696450011c546f6b656e4964000100205472616e736665720c01387472616e73616374696f6e5f696448010c753634000108746f18011c4163746f724964000120746f6b656e5f696450011c546f6b656e4964000200385472616e736665725061796f75741001387472616e73616374696f6e5f696448010c753634000108746f18011c4163746f724964000120746f6b656e5f696450011c546f6b656e4964000118616d6f756e7424011075313238000300244e46545061796f75740801146f776e657218011c4163746f724964000118616d6f756e74240110753132380004001c417070726f76650c01387472616e73616374696f6e5f696448010c753634000108746f18011c4163746f724964000120746f6b656e5f696450011c546f6b656e49640005004044656c656761746564417070726f76650c01387472616e73616374696f6e5f696448010c75363400011c6d65737361676558015c44656c656761746564417070726f76654d6573736167650001247369676e61747572655c01205b75383b2036345d000600144f776e6572040120746f6b656e5f696450011c546f6b656e4964000700284973417070726f766564080108746f18011c4163746f724964000120746f6b656e5f696450011c546f6b656e496400080014436c6561720401407472616e73616374696f6e5f6861736860011048323536000900244164644d696e7465720801387472616e73616374696f6e5f696448010c7536340001246d696e7465725f696418011c4163746f724964000a00004800000506004c1020676561725f6c6962486e6f6e5f66756e6769626c655f746f6b656e14746f6b656e34546f6b656e4d6574616461746100001001106e616d65080118537472696e6700012c6465736372697074696f6e080118537472696e670001146d65646961080118537472696e670001247265666572656e6365080118537472696e67000050083c7072696d69746976655f74797065731055323536000004005401205b7536343b20345d000054000003040000004800581020676561725f6c6962486e6f6e5f66756e6769626c655f746f6b656e2464656c6567617465645c44656c656761746564417070726f76654d6573736167650000140138746f6b656e5f6f776e65725f696418011c4163746f724964000144617070726f7665645f6163746f725f696418011c4163746f7249640001386e66745f70726f6772616d5f696418011c4163746f724964000120746f6b656e5f696450011c546f6b656e496400015065787069726174696f6e5f74696d657374616d7048010c75363400005c00000340000000200060083c7072696d69746976655f74797065731048323536000004001c01205b75383b2033325d00006408186e66745f696f204e46544576656e7400011c205472616e73666572040068012c4e46545472616e73666572000000385472616e736665725061796f757404006c01444e46545472616e736665725061796f7574000100244e46545061796f757404001401185061796f757400020020417070726f76616c040070012c4e4654417070726f76616c000300144f776e65720801146f776e657218011c4163746f724964000120746f6b656e5f696450011c546f6b656e4964000400284973417070726f7665640c0108746f18011c4163746f724964000120746f6b656e5f696450011c546f6b656e4964000120617070726f766564740110626f6f6c0005002c4d696e74657241646465640401246d696e7465725f696418011c4163746f72496400060000681020676561725f6c6962486e6f6e5f66756e6769626c655f746f6b656e08696f2c4e46545472616e7366657200000c011066726f6d18011c4163746f724964000108746f18011c4163746f724964000120746f6b656e5f696450011c546f6b656e496400006c1020676561725f6c6962486e6f6e5f66756e6769626c655f746f6b656e08696f444e46545472616e736665725061796f7574000010011066726f6d18011c4163746f724964000108746f18011c4163746f724964000120746f6b656e5f696450011c546f6b656e496400011c7061796f7574731401185061796f75740000701020676561725f6c6962486e6f6e5f66756e6769626c655f746f6b656e08696f2c4e4654417070726f76616c00000c01146f776e657218011c4163746f724964000140617070726f7665645f6163636f756e7418011c4163746f724964000120746f6b656e5f696450011c546f6b656e496400007400000500007808186e66745f696f14496f4e46540000100114746f6b656e7c0128496f4e46545374617465000120746f6b656e5f696450011c546f6b656e49640001146f776e657218011c4163746f7249640001307472616e73616374696f6e73a801545665633c28483235362c204e46544576656e74293e00007c08186e66745f696f28496f4e4654537461746500002001106e616d65080118537472696e6700011873796d626f6c080118537472696e67000120626173655f757269080118537472696e6700012c6f776e65725f62795f696480015c5665633c28546f6b656e49642c204163746f724964293e00013c746f6b656e5f617070726f76616c738801705665633c28546f6b656e49642c205665633c4163746f7249643e293e000150746f6b656e5f6d657461646174615f62795f69649001945665633c28546f6b656e49642c204f7074696f6e3c546f6b656e4d657461646174613e293e000140746f6b656e735f666f725f6f776e65729c01705665633c284163746f7249642c205665633c546f6b656e49643e293e000124726f79616c746965730c01444f7074696f6e3c526f79616c746965733e00008000000284008400000408501800880000028c008c0000040850400090000002940094000004085098009804184f7074696f6e040454014c0108104e6f6e6500000010536f6d6504004c00000100009c000002a000a00000040818a400a40000025000a8000002ac00ac00000408606400";

  const metadata = ProgramMetadata.from(meta);

  const currentaccount = account?.address;

  const getMyNFT = () => {
    api.programState
      .read({ programId: programIDNFT, payload: "" }, metadata)
      .then((result: any) => {
        setFullState(result.toJSON());

        const tokensForOwner: any = fullState.token.tokensForOwner ?? "";

        const tokenMetadataById: any = fullState.token.tokenMetadataById ?? "";

        tokenMetadataById.map((item: any) => alldatanfts.push(item[1]));

        setAllnfts(alldatanfts);

        setTokensForOwnerState(tokensForOwner);

        tokensForOwnerState.map((objeto: any) =>
          objeto[0] === decodeAddress(currentaccount ?? "")
            ? setAllmynft(objeto[1])
            : console.log("No NFT")
        );

        allmynft.forEach((posicion: any) => {
          if (posicion >= 0 && posicion < allnfts.length) {
            mynftscollection.push(allnfts[posicion]);
          }
        });

        setMynftcollection(mynftscollection);
      })
      .catch(({ message }: Error) => console.log(message));
  };

  getMyNFT();

  return (
    <div>
        <div style={{ width: "100%", height: "150px" }} />
        <div style={{ display: "flex", justifyContent: "center", alignItems: "center", flexDirection: "column" }}>
            <h3 style={{ color: "white", fontSize: "2rem" }}>My NFT Collection</h3>
            <div style={{ display: "flex" }}>
                {existNFT ? (
                    mynftcollection.map((elemento:any, index:any) => (
                        <button
                            key={crypto.randomUUID()}
                            type="button"
                            onClick={() => {
                                if (index === 0) {
                                    setMyselectednft(mynftcollection[index]);
                                } else if (index === 1) {
                                    setMyselectednft2(mynftcollection[index]);
                                } else if (index === 2) {
                                    setMyselectednft3(mynftcollection[index]);
                                }
                            }}
                            style={{ border: "none", background: "transparent", width: "100%", height: "100%" }}
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
                    <h3 style={{ fontSize: "1.5rem" }}>No NFTs</h3>
                )}
            </div>
        </div>
        <div style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
            <div style={{ display: "flex", gap: "10px" }}>
                <div style={{ width: "180px", height: "200px" }}>
                    <button
                    type="button"
                        onClick={() => {
                            firstinterruptor();
                        }}
                        style={{ backgroundColor: "green", border: "none", width: "100%", height: "100%" }}
                    >
                        <MySelectedNFT
                            name={myselectednft.name}
                            description={myselectednft.description}
                            media={myselectednft.media}
                            reference={myselectednft.reference}
                        />
                    </button>
                </div>
                <div style={{ width: "180px", height: "200px" }}>
                    <button
                    type="button"
                        onClick={() => {
                            secondinterruptor();
                        }}
                        style={{ backgroundColor: "green", border: "none", width: "100%", height: "100%" }}
                    >
                        <MySelectedNFT
                            name={myselectednft2.name}
                            description={myselectednft2.description}
                            media={myselectednft2.media}
                            reference={myselectednft2.reference}
                        />
                    </button>
                </div>

                <div style={{ width: "180px", height: "200px" }}>
                    <button
                    type="button"
                        onClick={() => {
                          thirdinterruptor();
                        }}
                        style={{ backgroundColor: "green", border: "none", width: "100%", height: "100%" }}
                    >
                        <MySelectedNFT
                            name={myselectednft3.name}
                            description={myselectednft3.description}
                            media={myselectednft3.media}
                            reference={myselectednft3.reference}
                        />
                    </button>
                </div>
            </div>
        </div>
    </div>
);

}

export { MainGame };