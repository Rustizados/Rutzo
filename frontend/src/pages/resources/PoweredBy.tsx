function PoweredBy() {
  return (
    <div className="m-5 lg:m-20 justify-center bg-gray-950 rounded-2xl text-center">
      <h1 className="text-3xl md:text-5xl font-semibold p-10 md:p-16">Powered by</h1>

      <div className="block justify-between mx-5 md:mx-60 p-5 lg:p-10 lg:flex">
        <div className="text-center">
          <img
            src="https://pbs.twimg.com/profile_images/1585274870136557569/QEvRxke1_400x400.jpg"
            alt="Vara Network"
            className="h-20 w-20 mx-auto rounded-xl"
          />
          <h2 className="text-xl m-5">Vara Network</h2>
        </div>

        <div>
          <img
            src="https://avatars.githubusercontent.com/u/137716679?s=280&v=4"
            alt="Gear"
            className="h-20 w-20 mx-auto rounded-xl"
          />
          <h2 className="text-xl m-5">Gear Foundation</h2>
        </div>
      </div>
    </div>
  );
}

export { PoweredBy };
