import React, { useState } from "react";
import "./EmptyCollection.scss";
import { Register } from "./Register";

function EmptyCollection() {
  const [isRegistered, setIsRegistered] = useState(false);
  const [mintCount, setMintCount] = useState(0);

  const handleRegister = () => {
    // Lógica para manejar el registro aquí
    setIsRegistered(true);
  };

  const handleMint = () => {
    if (mintCount < 3) {
      setMintCount(mintCount + 1);
    }
  };

  const renderMintButtons = () => {
    const mintButtons = [];
    for (let i = 0; i < mintCount; i+=1) {
      mintButtons.push(<button type="button" key={i}>Mint {i + 1}</button>);
    }
    return mintButtons;
  };

  return (
    <div className="container alert">
      <h1>Oops! it looks like you don&apos;t have any cards</h1>
      <img src="https://media.giphy.com/avatars/doodlesbyburnttoast/dMqxHmPPA8fd.gif" alt="empty" />
      {!isRegistered && (
        <>
          <p>Register to obtain free cards</p>
          <Register onRegister={handleRegister} />
        </>
      )}
      {isRegistered && mintCount < 3 && (
        <>
          <p>Select {3 - mintCount} more to mint:</p>
          <button type ="button" onClick={handleMint}>Mint</button>
        </>
      )}
      {mintCount > 0 && renderMintButtons()}
    </div>
  );
}

export { EmptyCollection };
