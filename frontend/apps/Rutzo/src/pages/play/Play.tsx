import { useEffect } from "react";
import { ReactComponent as ShoppingCart } from "@/assets/images/shopping_cart.svg";
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";
import { RegisterButton, MyNFTCollection, UserEmptyAccount } from "@/components";
import { ButtonToJoinInAGame } from "@/components/ComponentsForContracts/Actions/JoiningGame/ButtonToJoin";
import { ButtonToThrowCard } from "@/components/ComponentsForContracts/Actions/ThrowCard/ButtonToThrowCard";
import "./Collection.scss";
import useContractData from "@/hooks/useContractData";

function Play() {
  const { hasEnoughCards, fetchData, numberOfNfts, isRegister } = useContractData();

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  return (
      <div className="play-title">
        {isRegister ? (
            hasEnoughCards ? (
                <div className="alert">
                  <h1>Your Collection</h1>
                  <MyNFTCollection />
                  <br />
                  <div className="playcontainer">
                    <ButtonToJoinInAGame cardsId={[0, 1, 2]} playWithBot={false} />
                    <ButtonToThrowCard cardId={0} />
                    {/*TODO: Cambiar por un boton*/}
                    <a href="/game">
                      <GameController /> PLAY
                    </a>
                  </div>
                </div>
            ) : (
                numberOfNfts > 0 ? (
                    <div className="alert">
                      <h1>You don't have enough Cards</h1>
                      <MyNFTCollection />
                      <br />
                      <div className="playcontainer">
                        <a href="/marketplace">
                          <ShoppingCart /> MARKETPLACE
                        </a>
                      </div>
                    </div>
                ) : (
                    <UserEmptyAccount>
                      <p className="alert">Go to the marketplace and get some cool Cards!</p>
                      <div className="playcontainer">
                        <a href="/marketplace" className="alert">
                          MARKETPLACE
                        </a>
                      </div>
                    </UserEmptyAccount>
                )
            )
        ) : (
            <UserEmptyAccount>
              <p className="alert">Register to get free Cards</p>
              <RegisterButton onRegister={fetchData} />
            </UserEmptyAccount>
        )}
      </div>
  );
}

export { Play };
