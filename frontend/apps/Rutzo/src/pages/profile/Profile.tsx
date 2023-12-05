import "./Profile.scss";
import { useState } from "react";

function Profile() {
  const [loggedIn, setLoggedIn] = useState(false); // Estado para controlar si el usuario ha iniciado sesión o no

  const handleLogin = () => {
    setLoggedIn(true);
  };

  const handleLogout = () => {
    setLoggedIn(false);
  };

  const renderProfileDetails = () => {
    if (loggedIn) {
      return (
        <div>
          <h2>Detalles del usuario</h2>
          <button type="button" onClick={handleLogout}>
            Cerrar sesión
          </button>
        </div>
      );
    }

    return (
      <div>
        <p>No has iniciado sesión.</p>
        <button type="button" onClick={handleLogin}>
          Iniciar sesión
        </button>
      </div>
    );
  };

  return <div className="profile">{renderProfileDetails()}</div>;
}

export { Profile };
