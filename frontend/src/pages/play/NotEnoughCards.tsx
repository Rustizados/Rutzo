import { ReactComponent as ShoppingCart } from "@/assets/images/shopping_cart.svg";
import { RedirectionButton, MyNFTCollection } from "@/components";

function NotEnoughCards() {

  return (
    <div className="flex flex-col items-center justify-center">
      <h1 className=" text-3xl md:text-5xl font-semibold mb-6 ">
        You don't have enough cards!
      </h1>
      <p>You must have at least 3 cards to play, go to marketplace and get some cool NFTS to start playing</p>

      <MyNFTCollection />
      <br />

      <div className="playcontainer">
        <RedirectionButton
          style={{ marginTop: "3em", margin: "auto" }}
          link="/marketplace"
        >
          <ShoppingCart />
          MARKETPLACE
        </RedirectionButton>
      </div>
    </div>
  );
}

export { NotEnoughCards };
