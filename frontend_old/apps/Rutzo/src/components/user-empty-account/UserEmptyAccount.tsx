import "./UserEmptyAccount.scss";

function UserEmptyAccount({ children }: any) {
  return (
    <div className="empty_container">
      <h1 className="text-xl">Oops! it looks like you don&apos;t have any cards</h1>
      <img src="https://media.giphy.com/avatars/doodlesbyburnttoast/dMqxHmPPA8fd.gif" alt="empty" id="alert_img"
      className="rounded-md mx-auto" />
      { children }
    </div>
  );
}

export { UserEmptyAccount };
