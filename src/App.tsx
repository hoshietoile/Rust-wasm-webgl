import React, { useRef, useState } from 'react'
import './App.css'
import { Container } from './components/atoms/Container';
import { Panel } from './components/atoms/Panel';
import { Drawer } from './components/organisms/Drawer';
import { Header } from './components/organisms/Header';
import { Canvas } from './components/organisms/Canvas';
import { GameMaker } from './components/organisms/GameMaker';

function App() {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <div className="bg-white dark:bg-gray-700 h-full text-gray-700 dark:text-gray-300">
      {/* <div className="bg-emerald-700 h-full opacity-50"> */}
        <Header />
        <Container>
          <div>
            {/* <Panel> */}
              <GameMaker />
            {/* </Panel> */}
            {/* <button onClick={() => setIsOpen(true)}>
              {JSON.stringify(isOpen)}
            </button> */}
            {/* <Drawer isOpen={true} setIsOpen={(v) => setIsOpen(v)}>
              <Container className="w-64 bg-white">
                <div className="h-100">
                  hoge
                </div>
              </Container>
            </Drawer> */}
          </div>
        </Container>
      {/* </div> */}
    </div>
  )
}

export default App
