import { PlayButton } from "@/components";
import React, { useState, useEffect } from "react";
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";
import { RedirectionButton } from "@/components";

function PrepareToPlay() {
  const [currentImageIndex, setCurrentImageIndex] = useState(0);
  const [isToggleActive, setToggleActive] = useState(false);

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
        NFT{" "}
        <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl p-1">
          Battle
        </span>
      </h1>
      <p>Choose your better cards and may the odds be in your favor</p>

      <div className="flex items-center justify-center space-x-[-50px] mt-10">
        <img
          src={bannerImages[(currentImageIndex + 1) % 3]}
          alt="NFTs"
          style={{ transform: "rotate(-30deg)" }}
          className="border-2 border-white max-w-full h-auto rounded-lg w-32 z-10"
        />
        <img
          src={bannerImages[(currentImageIndex + 2) % 3]}
          alt="NFTs"
          className="border-2 border-white max-w-full h-auto rounded-lg w-32 -mt-12 z-20"
        />
        <img
          src={bannerImages[currentImageIndex]}
          alt="NFTs"
          style={{ transform: "rotate(30deg)" }}
          className="border-2 border-white max-w-full h-auto rounded-lg w-32 z-10"
        />
      </div>

      <div className="flex items-center justify-center rounded-full mt-10 w-72 h-10">
        <button className="text-xs flex items-center justify-center w-1/2 h-10 text-white bg-gray-950 hover:bg-gradient-to-r from-purple-800 to-green-400 rounded-l-full">
          Regular game
        </button>
        <button className="text-xs flex items-center justify-center w-1/2 h-10 text-white rounded-r-full bg-gray-950 hover:bg-gradient-to-r from-purple-800 to-green-400">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="currentColor"
            viewBox="0 0 24 24"
            stroke="currentColor"
            className="h-6 w-6 mr-2"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M13 10V3L4 14h7v7l9-11h-7z"
            />
          </svg>
          Quick game
        </button>
      </div>

      <div className="flex items-center justify-center mt-12">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="currentColor"
          viewBox="0 0 24 24"
          stroke="currentColor"
          className="h-6 w-6 mr-2"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M13 10V3L4 14h7v7l9-11h-7z"
          />
        </svg>
        <p className="mr-10">Enable signless</p>

        <button
          onClick={() => setToggleActive(!isToggleActive)}
          className={`ml-2 w-12 h-6 rounded-full ${
            isToggleActive
              ? "bg-gradient-to-r from-purple-800 to-green-400"
              : "bg-gray-300"
          }`}
        >
          <span
            className={`block w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-300 ease-in-out ${
              isToggleActive ? "translate-x-6" : ""
            }`}
          />
        </button>
      </div>

      <div className="playcontainer">
        <RedirectionButton
          style={{ marginTop: "3em", margin: "auto" }}
          link="/game"
        >
          <GameController />
          PLAY
        </RedirectionButton>
      </div>
    </div>
  );
}

export { PrepareToPlay };
