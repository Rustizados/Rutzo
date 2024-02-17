import { useEffect } from 'react';
import { ReactComponent as ShoppingCart } from '@/assets/images/shopping_cart.svg';
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";
import { RegisterButton, MyNFTCollection, UserEmptyAccount, RedirectionButton } from '@/components';
import useContractData from '@/hooks/useContractData';
import { Play as PlayButton } from '@/components/play/Play';

function Play() {
  const { hasEnoughCards, fetchData, numberOfNfts, isRegister } = useContractData();

  useEffect(() => {
    console.log("Se termino de renderizar, actualizando informacion");
    fetchData();
  }, [fetchData]);

  return (
    <div className="play-title">
      {isRegister ? (
        hasEnoughCards ? (
          <div className="alert">
            <h1 className=" text-3xl md:text-5xl font-semibold mb-6 ">
            Your <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl p-1">collection</span>
          </h1>
            <MyNFTCollection />
            <br />
            <div className="playcontainer">
              {/* <PlayButton style={{ marginInline: '20px', margin: 'auto' }} link="/game" /> */}
              <RedirectionButton style={{ marginInline: '20px', margin: 'auto' }} link="/game">
                <GameController />
                PLAY
              </RedirectionButton>
            </div>
          </div>
        ) : numberOfNfts > 0 ? (
          <div className="alert">
            <h1>You don't have enough Cards</h1>
            <MyNFTCollection />
            <br />
            <RedirectionButton style={{marginInline: "20px", height: "55px", display: "flex", justifyContent: "center", alignContent: "center"}} link="/marketplace">
              <ShoppingCart />
              Marketplace
            </RedirectionButton>
            {/* <div className="playcontainer">
              <a href="/marketplace">
                <ShoppingCart /> MARKETPLACE
              </a>
            </div> */}
          </div>
        ) : (
          <UserEmptyAccount>
            <p className="alert">Go to the marketplace and get some cool Cards!</p>
            <RedirectionButton style={{marginInline: "20px", height: "55px", display: "flex", justifyContent: "center", alignContent: "center"}} link="/marketplace">
              <ShoppingCart />
              Marketplace
            </RedirectionButton>
            {/* <div className="playcontainer">
              
              <a href="/marketplace" className="alert">
                MARKETPLACE
              </a>
            </div> */}
          </UserEmptyAccount>
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
