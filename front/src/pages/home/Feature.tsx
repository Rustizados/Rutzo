import React from 'react';
import Container from 'react-bootstrap/Container';
import './Feature.module.scss'; // Agrega los estilos necesarios aquí

interface FeatureProps {
  image: string;
  title: string;
  content: string;
}

function Feature({ image, title, content }: FeatureProps) {
  return (
    <Container className='feature'>
      <img src={image} style={{ width: '30%', height: '30%' }} alt="fireSpot" />
      <Container className='content'>
        <h2>{title}</h2>
        <p>{content}</p>
      </Container>
    </Container>
  );
}

export { Feature };
