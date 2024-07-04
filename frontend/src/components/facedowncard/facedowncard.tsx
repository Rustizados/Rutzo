import React from 'react';
import './facedowncard.css';

interface FacedowncardProps {
  scale?: number;
}

const Facedowncard: React.FC<FacedowncardProps> = ({ scale = 1 }) => {
  const scaledWidth = 13 * scale;
  const scaledHeight = 20 * scale;

  const cardStyle = {
    width: `${scaledWidth}rem`,
    height: `${scaledHeight}rem`,
  };

  return (
    <div className="card" style={cardStyle}>
      <img src="/BACK.png" alt="face down card" style={{ height: "100%" }} />
    </div>
  );
};

export { Facedowncard };
