import "./UserEmptyAccount.scss";
import { useState, useEffect } from "react";
import { RedirectionButton } from "@/components";
import { ReactComponent as ShoppingCart } from "@/assets/images/shopping_cart.svg";

function UserEmptyAccount() {
  const [currentImageIndex, setCurrentImageIndex] = useState(0);

  const bannerImages = [
    "https://home.rutzo.studio/NFT/poison/quetzal_poison.jpg",
    "https://home.rutzo.studio/NFT/lightning/nova_lighting.jpg",
    "https://home.rutzo.studio/NFT/poison/angel_of_death_poison.jpg",
  ];

  useEffect(() => {
    const intervalId = setInterval(() => {
      setCurrentImageIndex((prevIndex) =>
        prevIndex === bannerImages.length - 1 ? 0 : prevIndex + 1
      );
    }, 3000);

    return () => clearInterval(intervalId);
  }, [currentImageIndex, bannerImages.length]);

  return (
    <div className="flex flex-col items-center justify-center">
      <h1 className=" text-3xl md:text-5xl font-semibold mb-6 ">
        Oops! It looks like you don't have any cards
      </h1>
      <p>Go to the marketplace and get some cool Cards!</p>

      <div className="flex items-center justify-center space-x-[-50px] mt-10">
        <img
          src={bannerImages[(currentImageIndex + 1) % 3]}
          alt="NFTs"
          style={{ transform: "rotate(-30deg)" }}
          className="border-2 border-white max-w-full h-auto rounded-lg w-32"
        />
        <img
          src={bannerImages[(currentImageIndex + 2) % 3]}
          alt="NFTs"
          className="border-2 border-white max-w-full h-auto rounded-lg w-32 -mt-12 z-10"
        />
        <img
          src={bannerImages[currentImageIndex]}
          alt="NFTs"
          style={{ transform: "rotate(30deg)" }}
          className="border-2 border-white max-w-full h-auto rounded-lg w-32"
        />
      </div>
      <div className="mt-10">
        <RedirectionButton
          style={{
            marginInline: "20px",
            height: "55px",
            display: "flex",
            justifyContent: "center",
            alignContent: "center",
          }}
          link="/marketplace"
        >
          <ShoppingCart />
          Marketplace
        </RedirectionButton>
      </div>
    </div>
  );
}

export { UserEmptyAccount };
