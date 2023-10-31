import React, { useState } from "react";
import { MintNFT as Mint1 } from "components/card/Mint/MintNFT0";
import { MintNFT as Mint2 } from "components/card/Mint/MintNFT1";
import { MintNFT as Mint3 } from "components/card/Mint/MintNFT2";
import { MintNFT as Mint4 } from "components/card/Mint/MintNFT3";
import { MintNFT as Mint5 } from "components/card/Mint/MintNFT4";
import { MintNFT as Mint6 } from "components/card/Mint/MintNFT5";
import "./EmptyCollection.scss";
import { Register } from "./Register";

function EmptyCollection() {
  const [isRegistered, setIsRegistered] = useState(false);
  const [mintCount, setMintCount] = useState(0);
  const [mintButtonsState, setMintButtonsState] = useState([
    true, // Mint1 visible
    true, // Mint2 visible
    true, // Mint3 visible
    true, // Mint4 visible
    true, // Mint5 visible
    true  // Mint6 visible
  ]);

  const handleMintClick = (buttonIndex:any) => {
    if (buttonIndex < mintButtonsState.length && mintButtonsState[buttonIndex]) {
      // Lógica para manejar el clic en un botón de Mint aquí
      // Por ejemplo, realizar minting y luego ocultar el botón
      const updatedButtonsState = [...mintButtonsState];
      updatedButtonsState[buttonIndex] = false;
      setMintButtonsState(updatedButtonsState);
      setMintCount(mintCount + 1);
    }
  };

  const handleRegister = () => {
    // Lógica para manejar el registro aquí
    setIsRegistered(true);
  };

  return (
    <div className="empty_container">
      <h1>Oops! it looks like you don&apos;t have any cards</h1>
      <img src="https://media.giphy.com/avatars/doodlesbyburnttoast/dMqxHmPPA8fd.gif" alt="empty" id="alert_img" />
      {!isRegistered && (
        <>
          <p className="alert">Register to obtain free cards</p>
          <Register onRegister={handleRegister} className="playcontainer"/>
        </>
      )}
      {isRegistered && mintCount < 3 && (
        <>
          <p>Select {3 - mintCount} more to mint:</p>
          {mintButtonsState[0] && <button type="button" onClick={() => handleMintClick(0)}>Mint 1</button>}
          {mintButtonsState[1] && <button type="button" onClick={() => handleMintClick(1)}>Mint 2</button>}
          {mintButtonsState[2] && <button type="button" onClick={() => handleMintClick(2)}>Mint 3</button>}
          {mintButtonsState[3] && <button type="button" onClick={() => handleMintClick(3)}>Mint 4</button>}
          {mintButtonsState[4] && <button type="button" onClick={() => handleMintClick(4)}>Mint 5</button>}
          {mintButtonsState[5] && <button type="button" onClick={() => handleMintClick(5)}>Mint 6</button>}
        </>
      )}
    </div>
  );
}

export { EmptyCollection };
