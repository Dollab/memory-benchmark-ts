import React from "react";

import { useState } from 'react'
import './App.css'
import init, { WasmApp } from 'memory-bench';

const NUM_SEGMENTS = 1000000;

function App() {
  const [app, setApp] = useState<WasmApp | undefined>();
  const [segments, setSegments] = useState<Segment[]>([]);

  React.useEffect(() => {
    init().then(() => {
      console.log('init wasm-pack');
      const app = new WasmApp();
      setApp(app);
    });
  }, [])


  const createSegmentsInWasm = () => {
    if (!app) {
      return;
    }
    app.create_segments(NUM_SEGMENTS);
    console.log("Created 1 000 000 segments", app.allocated_size(), app.capacity());
  }

  const createSegmentsInJs = () => {
    const newSegments = segments.slice();
    for (let i = 0; i < NUM_SEGMENTS; i ++) {
      newSegments.push(new Segment());
    }
    setSegments(newSegments);
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>Create some segments</p>
        <p>
          <button type="button" onClick={() => createSegmentsInWasm()}>
            Create segments in wasm
          </button>
        </p>
        <p>
          <button type="button" onClick={() => createSegmentsInJs()}>
            Create segments in js
          </button>
        </p>
      </header>
    </div>
  )
}


class Segment {
  id: number
  start: [number, number]
  end: [number, number]
  color: [number, number, number, number]
  thickness: number
  transform: [number, number, number, number, number, number]
  zIndex: number

  // let's just initialize everything to zero per simplicity
  constructor() {
    this.id = 0;
    this.start = [0, 0]; // 8
    this.end = [0, 0]; // 8
    this.color = [0, 0, 0, 0]; // 16
    this.thickness = 0;
    this.transform = [0, 0, 0, 0, 0, 0]; // 24
    this.zIndex = 0;
  }
}

class RealisticSegment {
  start: Point
  end: Point
  color: Color
  transform: Transform

  constructor() {
    this.start = new Point();
    this.end = new Point();
    this.color = new Color();
    this.transform = new Transform();
  }
}

class Color {
  r:number
  g:number
  b:number
  a:number

  constructor() {
    this.r = Math.random();
    this.g = Math.random();
    this.b = Math.random();
    this.a = Math.random();
  }
}

class Transform {
  matrix: [number, number, number, number, number, number]

  constructor() {
    this.matrix = [Math.random(), Math.random(), Math.random(), Math.random(), Math.random(), Math.random()];
  }
}


class Point {
  x: number
  y: number

  constructor() {
    this.x = Math.random();
    this.y = Math.random();
  }
}

export default App
