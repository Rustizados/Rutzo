import { useEffect } from 'react';
import { ReactComponent as ShoppingCart } from '@/assets/images/shopping_cart.svg';
import { RegisterButton, MyNFTCollection, UserEmptyAccount } from '@/components';
import useContractData from '@/hooks/useContractData';
import { Play as PlayButton } from '@/components/play/Play';

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
            <h1 className=" text-3xl md:text-5xl font-semibold mb-6 ">
            Your <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">collection</span>
          </h1>
            <MyNFTCollection />
            <br />
            <div className="playcontainer">
              <PlayButton style={{ marginInline: '20px', margin: 'auto' }} link="/game" />
            </div>
          </div>
        ) : numberOfNfts > 0 ? (
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
      ) : (
        <UserEmptyAccount>
          <p className="text-base">Register to get free Cards</p>
          <RegisterButton onRegister={fetchData} />
        </UserEmptyAccount>
      )}
    </div>
  );
}

export { Play };
