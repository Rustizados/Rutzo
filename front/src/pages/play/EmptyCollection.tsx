import { Register } from "./Register";
import "./EmptyCollection.scss";

function EmptyCollection() {
  return (
    <div className="container">
      <h1>Oops! it looks like you don&apos;t have any cards</h1>
      <img src="https://media.giphy.com/avatars/doodlesbyburnttoast/dMqxHmPPA8fd.gif" alt="empty"/>
      <p>Register to obtain free cards</p>
      <Register />
    </div>
  );
}

export { EmptyCollection };
