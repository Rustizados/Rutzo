import { ProgramMetadata } from "@gear-js/api";
import { useAccount, useApi } from "@gear-js/react-hooks";
import { MAIN_CONTRACT } from "consts";
import { useState } from "react";
import { MyNFTCollection } from "./MyNFTCollection";
import { UnregisteredUser } from "./UnregisteredUser";


function Play() {
  const [isRegister, setIsRegister] = useState(false);
  const { api } = useApi();
  const { account } = useAccount();

  const setData = async () => {
    console.log("Si entro");
    
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
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

    console.log("Gas necesario para el programa: ");
    console.log(gas.toHuman().min_limit);
    
    console.log("Valor optenido async await: ");
    console.log(account?.decodedAddress);
    
    console.log(stateFormated);
    setIsRegister(stateFormated.userIsRegister);
  };

  setData();

  return (
    <div className="play-title">
      { 
        isRegister ? (
          <h1>User is reegister!!!</h1>
        ) : (
          <UnregisteredUser />
        )
      }
      {/* <MyNFTCollection /> */}
    </div>
  );
}

export { Play };


// return (
//   <div className="play-title">
//     <MyNFTCollection />
//   </div>
// );