import { RegisterButton } from "components";
import { ProgramMetadata } from "@gear-js/api";
import { useAccount, useApi } from "@gear-js/react-hooks";
import { MAIN_CONTRACT, NFT_CONTRACT } from "consts";
import { useState } from "react";
import { MyNFTCollection } from "./MyNFTCollection";
import { UserEmptyAccount } from "./UserEmptyAccount";


function Play() {
  const [userDoRegister, setUserDoRegister] = useState(false);
  const [hasEnoughCards, setHasEnoughCards] = useState(false);
  const [isRegister, setIsRegister] = useState(false);
  const { api } = useApi();
  const { account } = useAccount();
  const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
  const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

  const setData = async () => {    
    const stateResult = await api
      .programState
      .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
    const stateFormated: any = stateResult.toJSON();

    const gas = await api.program.calculateGas.handle(
      account?.decodedAddress ?? "0x00",
      MAIN_CONTRACT.PROGRAM_ID,
      { Register: null },
      0,
      false,
      mainContractMetadata
    );

    setIsRegister(stateFormated.userIsRegister);

    // if (!isRegister) return;

    

  };

  setData();

  return (
    <div className="play-title">
      { 
        isRegister ? (
          <>
            {  }
            <h1>User is reegister!!!</h1>
          </>
        ) : (
          <UserEmptyAccount>
            <p className="alert">Register to obtain free cards</p>
            <RegisterButton onRegister={setData} className="playcontainer" />
          </UserEmptyAccount>
        )
      }
    </div>
  );
}

export { Play };


// return (
//   <div className="play-title">
//     <MyNFTCollection />
//   </div>
// );