import { useEffect } from "react";
import { RegisterButton, UserEmptyAccount } from "@/components";
import useContractData from "@/hooks/useContractData";
import { Link } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import { increment } from "@/features/counterSlice";
import { PrepareToPlay } from "./PrepareToPlay";
import { NotEnoughCards } from "./NotEnoughCards";
import { NotRegistered } from "./NotRegistered";

function Play() {
  const dispatch = useDispatch();
  const count = useSelector((state: any) => state.counter.value);
  const { hasEnoughCards, fetchData, numberOfNfts, isRegister } =
    useContractData();

  useEffect(() => {
    console.log("Se termino de renderizar, actualizando informacion");
    fetchData();
  }, [fetchData]);

  return (
    <div className="play-title">
      {/*<button onClick={() => dispatch(increment())}>Increment</button>
      <Link to="/fight">Game</Link>
  <Link to="/select">Select</Link>

      <div>{count}</div>*/}
      {isRegister ? (
        hasEnoughCards ? (
          <PrepareToPlay />
        ) : numberOfNfts > 0 ? (
          <NotEnoughCards />
        ) : (
          <UserEmptyAccount />
        )
      ) : (
        <NotRegistered>
          <RegisterButton onRegister={fetchData} />
        </NotRegistered>
      )}
    </div>
  );
}

export { Play };
