import { Feature } from "./Feature";
import { FaCartShopping } from "react-icons/fa6";
import { IoBagHandle } from "react-icons/io5";
import { FaLayerGroup } from "react-icons/fa";


const featureData = [
  {
    id: 1,
    image: FaCartShopping,
    title: "Buy NFTs",
    content:
      "We provide you with a wide range of unique NFTs with different abilities to destroy your enemies! Each NFT possesses exceptional powers.",
  },
  {
    id: 2,
    image: IoBagHandle,
    title: "NFT Marketplace",
    content:
      "Defeat your enemies and conquer their NFTs to establish your dominance in the digital realm.",
  },
  {
    id: 3,
    image: FaLayerGroup,
    title: "Collect NFT",
    content:
      "Defeat your enemies and conquer their NFTs. As you emerge victorious in battles, claim the spoils of war by acquiring their valuable NFTs.",
  },
];

function Features() {
  return (
    <div id="Features">
      <div className="features-container">
        {featureData.map((feature) => (
          <Feature
            key={feature.id}
            image={feature.image}
            title={feature.title}
            content={feature.content}
          />
        ))}
      </div>
    </div>
  );
}

export { Features };
