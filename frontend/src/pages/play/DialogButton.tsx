import { useNavigate } from "react-router-dom";
import { useState } from "react";
import { SignlessDialog } from "./SignlessDialog";
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";

type DialogButtonProps = {
  link: string;
  isToggleActive?: boolean;
};

function DialogButton({ link, isToggleActive }: DialogButtonProps) {
  const [isDialogOpen, setDialogOpen] = useState(false);
  const navigate = useNavigate();

  function handleClick() {
    if (isToggleActive) {
      setDialogOpen(true);
    } else {
      navigate(link);
    }
  }

  return (
    <>
      <button onClick={handleClick} className="text-xs flex items-center justify-center w-56 h-10 text-white bg-gradient-to-r from-purple-800 to-green-400 rounded-full mt-10">
      <GameController className="w-5 mr-5"/>
        <h1 className="text-sm">PLAY</h1>
        </button>
      {isDialogOpen && <SignlessDialog onClose={() => setDialogOpen(false)} />}
    </>
  );
}

export { DialogButton };
