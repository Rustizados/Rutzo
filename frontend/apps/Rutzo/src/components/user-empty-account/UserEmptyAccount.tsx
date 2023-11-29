import React, { useState } from "react";
import "./UserEmptyAccount.scss";

function UserEmptyAccount({ children }: any) {
  return (
    <div className="empty_container">
      <h1>Oops! it looks like you don&apos;t have any cards</h1>
      <img src="https://media.giphy.com/avatars/doodlesbyburnttoast/dMqxHmPPA8fd.gif" alt="empty" id="alert_img" />
      { children }
    </div>
  );
}

export { UserEmptyAccount };
