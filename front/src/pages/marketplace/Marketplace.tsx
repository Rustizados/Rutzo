import { ReactComponent as Banner } from "assets/images/marketplace.svg";
import "./Marketplace.scss";
import { AllNFTCollection } from "./AllNFTCollection";
import { MyNFTCollection } from "./MyNFTCollection";

function Marketplace() {
  return (
    <div className="text-center">
      <Banner style={{ width: "50%", alignSelf: "center", padding: 0 }} />
      <h2 style={{ marginBottom: 80 }}>
        Get ready for the battle with some cool NFTs
      </h2>

      <div>
        <div className="cards-container">
          <MyNFTCollection />
          <br />
          <AllNFTCollection />
        </div>
      </div>
    </div>
  );
}

export { Marketplace };
