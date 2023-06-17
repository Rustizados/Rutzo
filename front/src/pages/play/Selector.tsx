import { Container, Row, Col, Button } from 'react-bootstrap';

function Selector() {
  return (
    <Container style={{ width: '100%' }}>
      <Row md={3}>
        <Col>
          <div className="border p-3" style={{ height: '300px', position: 'relative' }}>
            <h4>Container 1</h4>
            <Button variant="outline-primary" className="position-absolute bottom-0 end-0 m-3">
              <span>+</span>
            </Button>
          </div>
        </Col>
        <Col>
          <div className="border p-3" style={{ height: '300px', position: 'relative' }}>
            <h4>Container 2</h4>
            <Button variant="outline-primary" className="position-absolute bottom-0 end-0 m-3">
              <span>+</span>
            </Button>
          </div>
        </Col>
        <Col>
          <div className="border p-3" style={{ height: '300px', position: 'relative' }}>
            <h4>Container 3</h4>
            <Button variant="outline-primary" className="position-absolute bottom-0 end-0 m-3">
              <span>+</span>
            </Button>
          </div>
        </Col>
      </Row>

      <div className="playcontainer">
        <a href="/start-game" className="play">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={3.5} stroke="currentColor" className="w-6 h-6">
                <path strokeLinecap="round" strokeLinejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z" />
            </svg>
            Start game
        </a>
    </div>
    </Container>
  );
}

export { Selector };
